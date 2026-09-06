// SAFE: Uses crypto/rand to generate cryptographically secure random tokens.

package main

import (
	"crypto/rand"
	"encoding/hex"
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
	token := hex.EncodeToString(tokenBytes)
	err = saveResetToken(email, token)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "Reset link sent to %s", email)
}
