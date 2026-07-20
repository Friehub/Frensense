// SAFE: Uses filepath.Rel to verify the resolved path is within the intended directory.

package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
)

func readProfile(w http.ResponseWriter, r *http.Request) {
	profileDir := "/var/www/profiles"
	username := r.URL.Query().Get("user")
	resolved := filepath.Join(profileDir, username)
	rel, err := filepath.Rel(profileDir, resolved)
	if err != nil || strings.HasPrefix(rel, "..") {
		http.Error(w, "invalid path", http.StatusForbidden)
		return
	}
	data, err := os.ReadFile(resolved)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	fmt.Fprintf(w, "Profile: %s", data)
}
