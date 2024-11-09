package main

import (
	"database/sql"
	"fmt"
)

func UpdatePackageStatistics(db *sql.DB, upstreamName string, arch string, repo string, filename string) error {
	updateStmt, err := db.Prepare(`
		update cached_packages
		set
			download_count = download_count + 1,
			last_downloaded_at = strftime('%s', 'now')
		where upstream_id = (
			select id from upstreams where name = ? limit 1
		) and arch = ? and repo = ? and filename = ?`)
	if err != nil {
		return err
	}

	_, err = updateStmt.Exec(upstreamName, arch, repo, filename)
	return err
}

type Package struct {
	ID         int
	UpstreamID int
	Repo       string
	Arch       string
	Filename   string
}

type ShortUpstream struct {
	UpstreamID   int
	UpstreamName string
	Packages     []Package
}

// Upstream has Repos. Repo has Packages
func FetchPackageStatistics(db *sql.DB) ([]ShortUpstream, error) {
	var shortUpstreams []ShortUpstream
	shortUpstreamRows, err := db.Query(`
		select id, name from upstreams
	`)
	if err != nil {
		return nil, err
	}
	defer shortUpstreamRows.Close()
	for shortUpstreamRows.Next() {
		var shortUpstream ShortUpstream
		if err := shortUpstreamRows.Scan(&shortUpstream.UpstreamID, &shortUpstream.UpstreamName); err != nil {
			return nil, err
		}
		shortUpstream.Packages = make([]Package, 0)
		shortUpstreams = append(shortUpstreams, shortUpstream)
	}

	pkgRows, err := db.Query(`
		select id, upstream_id, arch, repo, filename from cached_packages
	`)
	if err != nil {
		return nil, err
	}
	defer pkgRows.Close()
	var packages []Package
	for pkgRows.Next() {
		var pkg Package
		if err := pkgRows.Scan(&pkg.ID, &pkg.UpstreamID, &pkg.Arch, &pkg.Repo, &pkg.Filename); err != nil {
			return nil, err
		}
		packages = append(packages, pkg)
	}

	for _, upstream := range shortUpstreams {
		for _, pkg := range packages {
			if pkg.UpstreamID == int(upstream.UpstreamID) {
				upstream.Packages = append(upstream.Packages, pkg)
			}
		}
		fmt.Printf("%d\n", len(upstream.Packages))
	}
	return shortUpstreams, nil
}
