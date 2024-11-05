package main

import (
	"database/sql"
	"fmt"
	"strings"
)

type UpstreamMirror struct {
	ID         int
	UpstreamID int
	Url        string
	CreatedAt  int
	UpdatedAt  *int
}

func (u UpstreamMirror) SubstituteUrlParams(arch string, repo string) string {
	v4Replaced := strings.ReplaceAll(u.Url, "$arch_v4", arch)
	v3Replaced := strings.ReplaceAll(v4Replaced, "$arch_v3", arch)
	vanillaReplaced := strings.ReplaceAll(v3Replaced, "$arch", arch)
	repoReplaced := strings.ReplaceAll(vanillaReplaced, "$repo", repo)
	return repoReplaced
}

func ShouldCacheFile(filename string) bool {
	return !(strings.Contains(filename, ".sig") || strings.Contains(filename, ".db"))
}

func GetMirrorsForUpstreamID(db *sql.DB, upstreamId int) ([]UpstreamMirror, error) {
	var mirrors []UpstreamMirror
	rows, err := db.Query("select id, upstream_id, url, created_at, updated_at from upstream_mirrors where upstream_id = ?", upstreamId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	for rows.Next() {
		var mirror UpstreamMirror
		if err := rows.Scan(&mirror.ID, &mirror.UpstreamID, &mirror.Url, &mirror.CreatedAt, &mirror.UpdatedAt); err != nil {
			return nil, err
		}
		mirrors = append(mirrors, mirror)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("albumsByArtist %q: %v", upstreamId, err)
	}

	return mirrors, nil
}
