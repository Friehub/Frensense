// SAFE: Rejects private and loopback IP ranges before making the HTTP request to prevent SSRF.

package main

import (
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
)

func isPrivateIP(host string) bool {
	ip := net.ParseIP(host)
	if ip == nil {
		return false
	}
	return ip.IsPrivate() || ip.IsLoopback() || ip.IsLinkLocalUnicast()
}

func fetchURL(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	parsed, err := url.Parse(target)
	if err != nil {
		http.Error(w, "invalid URL", http.StatusBadRequest)
		return
	}
	host := parsed.Hostname()
	if ips, err := net.LookupHost(host); err == nil {
		for _, ip := range ips {
			if isPrivateIP(ip) {
				http.Error(w, "URL not allowed", http.StatusForbidden)
				return
			}
		}
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
