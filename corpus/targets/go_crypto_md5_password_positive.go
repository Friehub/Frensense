// [frensense]
// observation: MD5 (crypto/md5) is used for password hashing, which is cryptographically broken and susceptible to collision and fast brute-force attacks.
// impact: An attacker who gains access to the password database can crack passwords quickly using rainbow tables or GPU-accelerated brute force because MD5 is designed for speed, not security.
// improvement: Use a dedicated password hashing algorithm like bcrypt, scrypt, or Argon2 with a cost factor and unique salt per password.

package handlers

import (
	"crypto/md5"
	"database/sql"
	"encoding/hex"
	"fmt"
	"net/http"
)

func registerUser(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	hash := md5.Sum([]byte(password))
	hashed := hex.EncodeToString(hash[:])
	_, err := db.Exec("INSERT INTO users (username, password_hash) VALUES (?, ?)", username, hashed)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "User %s created", username)
}
