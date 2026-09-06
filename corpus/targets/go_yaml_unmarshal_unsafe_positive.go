// [frensense]
// observation: yaml.Unmarshal from gopkg.in/yaml.v2 (or v3) is called on user-provided input without restrictions, allowing arbitrary type deserialization that can lead to code execution or resource exhaustion.
// impact: An attacker can craft a YAML document that exploits the parser to consume unbounded memory (YAML bomb) or instantiate arbitrary types, potentially leading to remote code execution.
// improvement: Use a strict YAML decoder with size limits, or validate the YAML structure before unmarshalling into known types.

package main

import (
	"fmt"
	"net/http"

	"gopkg.in/yaml.v2"
)

type Config struct {
	Name    string                 `yaml:"name"`
	Setting map[string]interface{} `yaml:"setting"`
}

func applyConfig(w http.ResponseWriter, r *http.Request) {
	body := r.Body
	defer body.Close()
	var cfg Config
	err := yaml.NewDecoder(body).Decode(&cfg)
	if err != nil {
		http.Error(w, "invalid config", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Applied: %s", cfg.Name)
}
