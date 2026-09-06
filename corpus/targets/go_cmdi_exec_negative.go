// SAFE: Uses exec.Command with separate arguments instead of shell invocation, preventing shell injection.

package main

import (
	"fmt"
	"net/http"
	"os/exec"
)

func compressLogs(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	cmd := exec.Command("gzip", "/var/log/"+filename)
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, fmt.Sprintf("error: %s", output), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "compressed: %s", filename)
}
