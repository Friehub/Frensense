// SAFE: Uses an allowlist of permitted external hosts to prevent SSRF.

package main

import (
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
)

var allowedHosts = map[string]bool{
	"api.example.com": true,
	"cdn.example.com": true,
}

func proxyRequest(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	parsed, err := url.Parse(target)
	if err != nil || !allowedHosts[parsed.Hostname()] {
		http.Error(w, "URL not allowed", http.StatusForbidden)
		return
	}
	client := &http.Client{
		Timeout: 10 * time.Second,
	}
	resp, err := client.Get(target)
	if err != nil {
		http.Error(w, "fetch failed", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Fprintf(w, "Response: %s", body)
}
