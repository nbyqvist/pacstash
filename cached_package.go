package main

import "database/sql"

type CachedPackage struct {
	ID               int
	UpstreamID       int
	Repo             string
	Arch             string
	Filename         string
	UpstreamMirrorID int
	CreatedAt        int
	UpdatedAt        *int
}

func FindCachedPackage(db *sql.DB, upstreamId int, repo string, arch string, filename string) (CachedPackage, error) {
	findPkgStmt, err := db.Prepare("select id, upstream_id, repo, arch, filename, upstream_mirror_id, created_at, updated_at from cached_packages where upstream_id = ? and repo = ? and arch = ? and filename = ? limit 1")
	if err != nil {
		return CachedPackage{}, err
	}

	cachedPackageRow := findPkgStmt.QueryRow(upstreamId, repo, arch, filename)
	var c CachedPackage
	if err = cachedPackageRow.Scan(&c.ID, &c.UpstreamID, &c.Repo, &c.Arch, &c.Filename, &c.UpstreamMirrorID, &c.CreatedAt, &c.UpdatedAt); err != nil {
		return CachedPackage{}, err
	}

	return c, nil
}

func CreateCachedPackage(db *sql.DB, upstreamId int, upstreamMirrorId int, arch string, repo string, filename string) error {
	insertPkgStmt, err := db.Prepare("insert into cached_packages (upstream_id, repo, arch, filename, upstream_mirror_id) values (?, ?, ?, ?, ?)")
	if err != nil {
		return err
	}

	_, err = insertPkgStmt.Exec(upstreamId, repo, arch, filename, upstreamMirrorId)
	if err != nil {
		return err
	}

	return nil
}
