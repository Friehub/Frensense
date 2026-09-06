// SAFE: Uses prepared statement with parameter binding, preventing SQL injection.

package main

import (
	"database/sql"
	"fmt"
	"net/http"
)

func getUserByID(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	stmt, err := db.Prepare("SELECT * FROM users WHERE id = ?")
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}
	defer stmt.Close()
	row := stmt.QueryRow(id)
	var user User
	row.Scan(&user.ID, &user.Name, &user.Email)
	fmt.Fprintf(w, "User: %s", user.Name)
}
