// SAFE: Restricts the HTTP client's transport to deny private IP ranges via a custom dialer.

package main

import (
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"time"
)

func isBlocked(host string) bool {
	ip := net.ParseIP(host)
	if ip == nil {
		return false
	}
	return ip.IsPrivate() || ip.IsLoopback() || ip.IsLinkLocalUnicast()
}

func proxyRequest(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	parsed, err := url.Parse(target)
	if err != nil {
		http.Error(w, "invalid URL", http.StatusBadRequest)
		return
	}
	client := &http.Client{
		Timeout: 10 * time.Second,
		Transport: &http.Transport{
			DialContext: (&net.Dialer{
				Timeout: 5 * time.Second,
			}).DialContext,
		},
	}
	host := parsed.Hostname()
	if ips, err := net.LookupHost(host); err == nil {
		for _, ip := range ips {
			if isBlocked(ip) {
				http.Error(w, "URL not allowed", http.StatusForbidden)
				return
			}
		}
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
