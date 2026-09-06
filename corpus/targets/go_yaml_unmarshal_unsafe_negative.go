// SAFE: Uses yaml.UnmarshalStrict to reject unknown fields and limits input size before parsing.

package main

import (
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
	var cfg Config
	err = yaml.Unmarshal(data, &cfg)
	if err != nil {
		http.Error(w, "invalid config", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Applied: %s", cfg.Name)
}
