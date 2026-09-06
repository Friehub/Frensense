// SAFE: Fetches the resource first, then verifies ownership before returning it.

package main

import (
	"database/sql"
	"encoding/json"
	"net/http"
)

type Invoice struct {
	ID     int     `json:"id"`
	Amount float64 `json:"amount"`
	OwnerID int    `json:"owner_id"`
}

func getInvoice(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		userID := r.Context().Value("user_id").(int)
		invoiceID := r.URL.Query().Get("id")
		var inv Invoice
		err := db.QueryRow("SELECT id, amount, owner_id FROM invoices WHERE id = ?", invoiceID).
			Scan(&inv.ID, &inv.Amount, &inv.OwnerID)
		if err != nil {
			http.Error(w, "not found", http.StatusNotFound)
			return
		}
		if inv.OwnerID != userID {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		json.NewEncoder(w).Encode(inv)
	}
}
