// [frensense]
// observation: User-controlled URL is passed directly to http.Get without validation, allowing the server to make requests to internal or unintended hosts.
// impact: An attacker can use the server as a proxy to scan internal networks, access cloud metadata endpoints (169.254.169.254), or bypass firewall restrictions.
// improvement: Validate the URL against an allowlist of permitted hosts or URL schemes before making the request.

package main

import (
	"fmt"
	"io"
	"net/http"
)

func fetchURL(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Fprintf(w, "Response: %s", body)
}
