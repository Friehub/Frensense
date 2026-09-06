// [frensense]
// observation: User input is passed to exec.Command with bash -c, allowing shell metacharacters to inject arbitrary commands.
// impact: An attacker can execute arbitrary OS commands on the server by injecting shell metacharacters (;, &, |, $(), ``) into the input.
// improvement: Avoid shell invocation entirely; use direct command execution with separate argument slices.

package main

import (
	"fmt"
	"net/http"
	"os/exec"
)

func compressLogs(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	cmd := exec.Command("bash", "-c", fmt.Sprintf("gzip /var/log/%s", filename))
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, fmt.Sprintf("error: %s", output), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "compressed: %s", filename)
}
