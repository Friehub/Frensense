// SAFE: Uses parameterized query with ? placeholders, preventing SQL injection.

package main

import (
	"database/sql"
	"fmt"
	"net/http"
)

func getUserByID(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	row := db.QueryRow("SELECT * FROM users WHERE id = ?", id)
	var user User
	row.Scan(&user.ID, &user.Name, &user.Email)
	fmt.Fprintf(w, "User: %s", user.Name)
}
