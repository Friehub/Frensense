// SAFE: Uses exec.Command with allowlisted tool names instead of raw os.StartProcess.

package main

import (
	"fmt"
	"net/http"
	"os/exec"
)

var allowedTools = map[string]string{
	"ping":  "ping",
	"netstat": "ss",
	"df":    "df",
}

func runDiagnostic(w http.ResponseWriter, r *http.Request) {
	tool := r.URL.Query().Get("tool")
	bin, ok := allowedTools[tool]
	if !ok {
		http.Error(w, "unknown tool", http.StatusBadRequest)
		return
	}
	cmd := exec.Command(bin, "--check")
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "output: %s", output)
}
