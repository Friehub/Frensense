// SAFE: Uses Argon2id for password hashing, the current state-of-the-art password hash.

package handlers

import (
	"crypto/rand"
	"database/sql"
	"encoding/base64"
	"fmt"
	"net/http"

	"golang.org/x/crypto/argon2"
)

func registerUser(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	salt := make([]byte, 16)
	rand.Read(salt)
	hash := argon2.IDKey([]byte(password), salt, 1, 64*1024, 4, 32)
	encoded := base64.RawStdEncoding.EncodeToString(salt) + "$" + base64.RawStdEncoding.EncodeToString(hash)
	_, err := db.Exec("INSERT INTO users (username, password_hash) VALUES (?, ?)", username, encoded)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "User %s created", username)
}
