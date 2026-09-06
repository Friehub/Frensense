// SAFE: Cleans the path with filepath.Clean and verifies the result is within the base directory.

package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func serveFile(w http.ResponseWriter, r *http.Request) {
	baseDir := "/var/www/files/"
	filename := r.URL.Query().Get("name")
	fullPath := filepath.Clean(baseDir + filename)
	if !strings.HasPrefix(fullPath, filepath.Clean(baseDir)) {
		http.Error(w, "invalid path", http.StatusForbidden)
		return
	}
	data, err := os.ReadFile(fullPath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	fmt.Fprintf(w, "File: %s", data)
}
