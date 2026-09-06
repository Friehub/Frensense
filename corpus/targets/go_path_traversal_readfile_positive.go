// [frensense]
// observation: User input is concatenated into a file path and passed to os.ReadFile without sanitization, allowing path traversal via ../ sequences.
// impact: An attacker can read arbitrary files on the server, including configuration files, credentials, and sensitive data outside the intended directory.
// improvement: Use filepath.Clean to normalize paths and verify the resolved path is within the intended base directory before reading.

package main

import (
	"fmt"
	"net/http"
	"os"
)

func serveFile(w http.ResponseWriter, r *http.Request) {
	baseDir := "/var/www/files/"
	filename := r.URL.Query().Get("name")
	data, err := os.ReadFile(baseDir + filename)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	fmt.Fprintf(w, "File: %s", data)
}
