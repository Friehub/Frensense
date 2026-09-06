// SAFE: Verifies the resolved path stays within the intended base directory using prefix check.

package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func readProfile(w http.ResponseWriter, r *http.Request) {
	profileDir := "/var/www/profiles"
	username := r.URL.Query().Get("user")
	filePath := filepath.Join(profileDir, username)
	if !strings.HasPrefix(filePath, profileDir) {
		http.Error(w, "invalid path", http.StatusForbidden)
		return
	}
	data, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	fmt.Fprintf(w, "Profile: %s", data)
}
