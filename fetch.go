package main

import (
	"errors"
	"fmt"
	"io"
	"net/http"
)

func FetchPackage(mirrors []UpstreamMirror, arch string, repo string, filename string) ([]byte, int, error) {
	// Ideally the mirrors would be ranked, but meh
	for _, mirror := range mirrors {
		mirrorUrl := fmt.Sprintf("%s/%s", mirror.SubstituteUrlParams(arch, repo), filename)
		resp, err := http.Get(mirrorUrl)
		if err != nil {
			fmt.Printf("Mirror url %s returned error %v\n", mirrorUrl, err)
			continue
		}
		if resp.StatusCode != 200 {
			fmt.Printf("Mirror url %s returned code %d\n", mirrorUrl, resp.StatusCode)
			continue
		}
		defer resp.Body.Close()
		content, err := io.ReadAll(resp.Body)
		if err != nil {
			fmt.Printf("Error decoding file from mirror url %s, %v\n", mirror.Url, err)
			continue
		}
		return content, mirror.ID, nil
	}

	return nil, 0, errors.New("all mirrors failed to fetch")
}
