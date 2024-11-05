package main

import "strings"

func ShouldCacheFile(filename string) bool {
	return strings.Contains(filename, ".sig") || strings.Contains(filename, ".db")
}
