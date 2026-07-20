// [frensense]
// observation: The default Go XML parser (encoding/xml) with Decoder allows external entity expansion, which can lead to XXE attacks when parsing user-supplied XML.
// impact: An attacker can include external entities in the XML to read local files (e.g., /etc/passwd), perform SSRF to internal networks, or cause denial of service via billion laughs attack.
// improvement: Disable external entity expansion by setting a custom XMLDecoder with entity limiting or pre-processing the input.

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
	err := decoder.Decode(&doc)
	if err != nil {
		http.Error(w, "invalid XML", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Content: %s", doc.Content)
}
