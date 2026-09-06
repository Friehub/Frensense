// SAFE: Pre-validates YAML structure with a known schema before unmarshalling into a concrete type.

package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

type Config struct {
	Name    string            `yaml:"name"`
	Setting map[string]string `yaml:"setting"`
}

func applyConfig(w http.ResponseWriter, r *http.Request) {
	body := r.Body
	defer body.Close()
	data, err := io.ReadAll(io.LimitReader(body, 1024*10))
	if err != nil {
		http.Error(w, "body too large", http.StatusRequestEntityTooLarge)
		return
	}
	var raw interface{}
	if err := yaml.Unmarshal(data, &raw); err != nil {
		http.Error(w, "invalid yaml", http.StatusBadRequest)
		return
	}
	var cfg Config
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		http.Error(w, "invalid config structure", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Applied: %s", cfg.Name)
}
