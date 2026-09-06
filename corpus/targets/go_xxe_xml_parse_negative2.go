// SAFE: Pre-processes the XML input to strip DOCTYPE declarations and entity definitions before parsing.

package main

import (
	"bytes"
	"encoding/xml"
	"fmt"
	"io"
	"net/http"
	"strings"
)

type Document struct {
	Content string `xml:"content"`
}

func stripXMLDoctype(input io.Reader) io.Reader {
	data, _ := io.ReadAll(input)
	cleaned := string(data)
	if idx := strings.Index(cleaned, "<!DOCTYPE"); idx >= 0 {
		endIdx := strings.Index(cleaned[idx:], ">")
		if endIdx >= 0 {
			cleaned = cleaned[:idx] + cleaned[idx+endIdx+1:]
		}
	}
	return bytes.NewReader([]byte(cleaned))
}

func parseXML(w http.ResponseWriter, r *http.Request) {
	body := r.Body
	defer body.Close()
	safeInput := stripXMLDoctype(body)
	var doc Document
	decoder := xml.NewDecoder(safeInput)
	err := decoder.Decode(&doc)
	if err != nil {
		http.Error(w, "invalid XML", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Content: %s", doc.Content)
}
