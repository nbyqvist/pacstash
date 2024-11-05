package main

import (
	"fmt"
	"os"
	"path"
	"strings"
)

type CacheEntry struct {
	UpstreamName string
	Repository   string
	Architecture string
	FileName     string
}

func (c CacheEntry) ToPath() string {
	return path.Join(c.UpstreamName, c.Repository, c.Architecture, c.FileName)
}

type FilesystemCacheManager struct {
	RootPath string
}

func (f FilesystemCacheManager) ReadFile(entry CacheEntry) ([]byte, error) {
	entryPath := path.Join(f.RootPath, entry.ToPath())
	content, err := os.ReadFile(entryPath)
	return content, err
}

func (f FilesystemCacheManager) WriteFile(entry CacheEntry, content []byte) error {
	entryPath := path.Join(f.RootPath, entry.ToPath())
	entryBaseDir := strings.ReplaceAll(entryPath, fmt.Sprintf("/%s", entry.FileName), "")
	err := os.MkdirAll(entryBaseDir, 0700)
	if err != nil {
		return err
	}

	err = os.WriteFile(entryPath, content, os.FileMode(0o644))
	return err
}

func (f FilesystemCacheManager) PathOfCachedPackage(upstreamName string, cachedPackage CachedPackage) string {
	return path.Join(f.RootPath, upstreamName, cachedPackage.Repo, cachedPackage.Arch, cachedPackage.Filename)
}
