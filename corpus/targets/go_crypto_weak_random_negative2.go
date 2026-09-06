// SAFE: Uses crypto/rand with base64 encoding for URL-safe secure tokens.

package main

import (
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"net/http"
)

func requestPasswordReset(w http.ResponseWriter, r *http.Request) {
	email := r.FormValue("email")
	tokenBytes := make([]byte, 32)
	_, err := rand.Read(tokenBytes)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	token := base64.URLEncoding.EncodeToString(tokenBytes)
	err = saveResetToken(email, token)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "Reset link sent to %s", email)
}
