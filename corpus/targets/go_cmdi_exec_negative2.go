// SAFE: Removes shell invocation and uses stdlib file operations instead of command execution.

package main

import (
	"compress/gzip"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
)

func compressLogs(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	safePath := filepath.Clean("/var/log/" + filename)
	if !strings.HasPrefix(safePath, "/var/log/") {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	f, err := os.Open(safePath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	defer f.Close()
	gz, err := gzip.NewWriterLevel(f, gzip.BestCompression)
	if err != nil {
		http.Error(w, "compression failed", http.StatusInternalServerError)
		return
	}
	gz.Close()
	fmt.Fprintf(w, "compressed: %s", filename)
}
