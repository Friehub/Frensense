// SAFE: Uses parameterized query with ? placeholder, preventing SQL injection.

package main

import (
	"database/sql"
	"fmt"
	"net/http"
)

func searchProducts(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	search := r.URL.Query().Get("q")
	rows, err := db.Query("SELECT * FROM products WHERE name LIKE ?", "%"+search+"%")
	if err != nil {
		http.Error(w, "search failed", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	for rows.Next() {
		var p Product
		rows.Scan(&p.ID, &p.Name, &p.Price)
		fmt.Fprintf(w, "%s: $%d\n", p.Name, p.Price)
	}
}
