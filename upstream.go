package main

import "database/sql"

type Upstream struct {
	ID           int
	Name         string
	UpstreamType string
	CreatedAt    int
	UpdatedAt    *int
}

func FindUpstreamByName(db *sql.DB, name string) (Upstream, error) {
	findStmt, err := db.Prepare("select id, name, upstream_id, created_at, updated_at from upstreams where name = ? limit 1")
	if err != nil {
		return Upstream{}, err
	}

	upstreamRow := findStmt.QueryRow(name)
	var upstream Upstream
	if err = upstreamRow.Scan(&upstream.ID, &upstream.Name, &upstream.UpstreamType, &upstream.CreatedAt, &upstream.UpdatedAt); err != nil {
		return Upstream{}, err
	}

	return upstream, nil
}
