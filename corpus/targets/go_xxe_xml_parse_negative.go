// SAFE: Replaces the XML decoder with a strict decoder that does not expand external entities.

package main

import (
	"encoding/xml"
	"fmt"
	"net/http"
	"strings"
)

type Document struct {
	Content string `xml:"content"`
}

func parseXML(w http.ResponseWriter, r *http.Request) {
	body := r.Body
	defer body.Close()
	var doc Document
	decoder := xml.NewDecoder(body)
	decoder.Strict = true
	decoder.AutoClose = xml.HTMLAutoClose
	decoder.Entity = xml.HTMLEntity
	err := decoder.Decode(&doc)
	if err != nil {
		http.Error(w, "invalid XML", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Content: %s", doc.Content)
}
