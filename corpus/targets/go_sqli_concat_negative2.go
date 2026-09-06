// SAFE: Uses sqlx named parameters to prevent SQL injection with safe parameter binding.

package main

import (
	"fmt"
	"net/http"

	"github.com/jmoiron/sqlx"
)

func searchProducts(db *sqlx.DB, w http.ResponseWriter, r *http.Request) {
	search := r.URL.Query().Get("q")
	rows, err := db.NamedQuery("SELECT * FROM products WHERE name LIKE :search", map[string]interface{}{
		"search": "%" + search + "%",
	})
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
