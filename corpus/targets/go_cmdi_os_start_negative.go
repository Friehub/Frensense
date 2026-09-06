// SAFE: Validates the tool name against an allowlist before starting the process.

package main

import (
	"fmt"
	"net/http"
	"os"
)

var allowedTools = map[string]string{
	"ping":  "/usr/bin/ping",
	"netstat": "/usr/bin/ss",
	"df":    "/usr/bin/df",
}

func runDiagnostic(w http.ResponseWriter, r *http.Request) {
	tool := r.URL.Query().Get("tool")
	binPath, ok := allowedTools[tool]
	if !ok {
		http.Error(w, "unknown tool", http.StatusBadRequest)
		return
	}
	attr := &os.ProcAttr{
		Files: []*os.File{os.Stdin, os.Stdout, os.Stderr},
	}
	proc, err := os.StartProcess(binPath, []string{binPath, "--check"}, attr)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	state, err := proc.Wait()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "exit code: %d", state.ExitCode())
}
