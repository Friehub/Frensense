// SAFE: Validates the URL against an allowlist of permitted external hosts before making the request.

package main

import (
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
)

var allowedHosts = map[string]bool{
	"api.example.com": true,
	"data.example.com": true,
}

func fetchURL(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	parsed, err := url.Parse(target)
	if err != nil || !allowedHosts[parsed.Hostname()] {
		http.Error(w, "URL not allowed", http.StatusForbidden)
		return
	}
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch failed", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Fprintf(w, "Response: %s", body)
}
