// SAFE: Uses filepath.Join for safe path construction and verifies the resolved path stays within base directory.

package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
)

func serveFile(w http.ResponseWriter, r *http.Request) {
	baseDir := "/var/www/files"
	filename := r.URL.Query().Get("name")
	fullPath := filepath.Join(baseDir, filename)
	if !strings.HasPrefix(fullPath, baseDir) {
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
