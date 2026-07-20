// [frensense]
// observation: User input is concatenated directly into a SQL query string, allowing SQL injection via crafted input with SQL metacharacters.
// impact: An attacker can manipulate the query structure to read, modify, or delete arbitrary data from the database.
// improvement: Use parameterized queries or an ORM with safe query building to separate SQL code from data.

package main

import (
	"database/sql"
	"fmt"
	"net/http"
)

func searchProducts(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	search := r.URL.Query().Get("q")
	query := "SELECT * FROM products WHERE name LIKE '%" + search + "%'"
	rows, err := db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	for rows.Next() {
		var p Product
		rows.Scan(&p.ID, &p.Name, &p.Price)
		fmt.Fprintf(w, "%s: $%d\n", p.Name, p.Price)
	}
}
