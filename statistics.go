package main

import "database/sql"

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
