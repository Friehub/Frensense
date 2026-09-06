// [frensense]
// observation: A custom http.Client is used with a user-controlled URL without any validation, allowing SSRF attacks to internal services.
// impact: An attacker can use the HTTP client to make requests to internal services, cloud metadata endpoints, or other restricted networks.
// improvement: Add URL validation against an allowlist before passing the URL to the client, or restrict the transport's dialer.

package main

import (
	"fmt"
	"io"
	"net/http"
	"time"
)

func proxyRequest(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	client := &http.Client{
		Timeout: 10 * time.Second,
	}
	resp, err := client.Get(target)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Fprintf(w, "Response: %s", body)
}
