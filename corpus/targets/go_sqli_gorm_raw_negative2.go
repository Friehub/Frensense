// SAFE: Uses GORM's Where method with parameterized condition, avoiding raw SQL entirely.

package main

import (
	"fmt"
	"net/http"

	"gorm.io/gorm"
)

func getUserByEmail(db *gorm.DB, w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	var user User
	db.Where("email = ?", email).First(&user)
	fmt.Fprintf(w, "User: %s", user.Name)
}
