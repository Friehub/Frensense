// [frensense]
// observation: User-controlled input is passed to os.StartProcess as the process path, allowing an attacker to execute arbitrary binaries on the server.
// impact: An attacker can start any executable on the system by controlling the process name or arguments, leading to full system compromise.
// improvement: Validate the process name against an allowlist and sanitize arguments before passing to StartProcess.

package main

import (
	"fmt"
	"net/http"
	"os"
	"os/exec"
)

func runDiagnostic(w http.ResponseWriter, r *http.Request) {
	tool := r.URL.Query().Get("tool")
	attr := &os.ProcAttr{
		Files: []*os.File{os.Stdin, os.Stdout, os.Stderr},
	}
	proc, err := os.StartProcess(tool, []string{tool, "--check"}, attr)
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
