// SAFE: Uses GORM Raw with parameterized placeholders, preventing SQL injection.

package main

import (
	"fmt"
	"net/http"

	"gorm.io/gorm"
)

func getUserByEmail(db *gorm.DB, w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	var user User
	db.Raw("SELECT * FROM users WHERE email = ?", email).Scan(&user)
	fmt.Fprintf(w, "User: %s", user.Name)
}
