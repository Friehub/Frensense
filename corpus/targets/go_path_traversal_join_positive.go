// [frensense]
// observation: filepath.Join is used to construct a path from user input, but the result is not checked to ensure it stays within the intended base directory.
// impact: An attacker can use ../ sequences in the user input to escape the base directory and read or write files anywhere on the filesystem.
// improvement: After joining, verify the resolved absolute path has the expected prefix using strings.HasPrefix or filepath.Rel.

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
	filePath := filepath.Join(profileDir, username)
	data, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	fmt.Fprintf(w, "Profile: %s", data)
}
