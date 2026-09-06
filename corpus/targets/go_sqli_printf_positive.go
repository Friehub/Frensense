// [frensense]
// observation: User input is interpolated directly into a SQL query via fmt.Sprintf, allowing SQL injection through crafted input containing quotes or escape characters.
// impact: An attacker can bypass authentication, exfiltrate data, or execute arbitrary SQL commands on the database.
// improvement: Use parameterized queries with placeholders (? for MySQL/SQLite, $1 for PostgreSQL) instead of string formatting.

package main

import (
	"database/sql"
	"fmt"
	"net/http"
)

func getUserByID(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = '%s'", id)
	row := db.QueryRow(query)
	var user User
	row.Scan(&user.ID, &user.Name, &user.Email)
	fmt.Fprintf(w, "User: %s", user.Name)
}
