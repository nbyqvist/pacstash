package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/glebarez/go-sqlite"
	"github.com/gofiber/fiber/v2"
)

func main() {
	db, err := sql.Open("sqlite", "dev.db")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Starting up pacstash...")

	fsCacheMan := FilesystemCacheManager{
		RootPath: "./fake_cache",
	}
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("pacstash. This will return stats")
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
			} else {
				// Cache hit
				return c.SendFile(fsCacheMan.PathOfCachedPackage(upstreamName, cachedPackage))
			}
		}

		// Proxy request upstream
	})

	log.Fatal(app.Listen("0.0.0.0:3000"))
}
