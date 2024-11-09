package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/glebarez/go-sqlite"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/html/v2"
)

func main() {
	dbFile := os.Getenv("DATABASE")
	webPort := os.Getenv("WEB_PORT")
	webHost := os.Getenv("WEB_HOST")
	cacheRoot := os.Getenv("CACHE_ROOT")

	engine := html.New("./views", ".html")

	db, err := sql.Open("sqlite", dbFile)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Starting up pacstash...")

	fsCacheMan := FilesystemCacheManager{
		RootPath: cacheRoot,
	}
	app := fiber.New(fiber.Config{
		Views: engine,
	})

	app.Get("/", func(c *fiber.Ctx) error {
		stats, err := FetchPackageStatistics(db)
		if err != nil {
			return err
		}
		return c.Render("stats", fiber.Map{
			"stats": stats,
		})
	})

	app.Get("/u/:upstream_name/:repo/:arch/:filename", func(c *fiber.Ctx) error {
		upstreamName := c.Params("upstream_name")
		upstream, err := FindUpstreamByName(db, upstreamName)
		if err != nil {
			return err
		}

		repo := c.Params("repo")
		arch := c.Params("arch")
		filename := c.Params("filename")
		if ShouldCacheFile(filename) {
			cachedPackage, err := FindCachedPackage(db, upstream.ID, repo, arch, filename)
			if err != nil && err != sql.ErrNoRows {
				// Other error
				return err
			} else if err == sql.ErrNoRows {
				// Cache miss
				mirrors, err := GetMirrorsForUpstreamID(db, upstream.ID)
				if err != nil {
					return err
				}
				pkg, mirrorId, err := FetchPackage(mirrors, arch, repo, filename)
				if err != nil {
					return err
				}
				err = CreateCachedPackage(db, upstream.ID, mirrorId, arch, repo, filename)
				if err != nil {
					return err
				}

				err = fsCacheMan.WriteFile(CacheEntry{Architecture: arch, Repository: repo, FileName: filename, UpstreamName: upstreamName}, pkg)
				if err != nil {
					return err
				}
				c.Set("Content-Type", "application/octet-stream")
				return c.Send(pkg)
			} else {
				// Cache hit
				err := UpdatePackageStatistics(db, upstreamName, arch, repo, filename)
				if err != nil {
					return err
				}
				return c.SendFile(fsCacheMan.PathOfCachedPackage(upstreamName, cachedPackage))
			}
		} else {
			// Proxy request upstream
			mirrors, err := GetMirrorsForUpstreamID(db, upstream.ID)
			if err != nil {
				return err
			}
			pkg, _, err := FetchPackage(mirrors, arch, repo, filename)
			if err != nil {
				return err
			}
			c.Set("Content-Type", "application/octet-stream")
			return c.Send(pkg)
		}
	})

	webListenAddr := fmt.Sprintf("%s:%s", webHost, webPort)

	log.Fatal(app.Listen(webListenAddr))
}
