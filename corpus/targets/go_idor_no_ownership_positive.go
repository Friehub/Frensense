// [frensense]
// observation: The handler retrieves a resource by ID from user input but does not verify that the authenticated user owns the resource before returning it.
// impact: An attacker can enumerate or access any user's resources by changing the ID parameter, leading to unauthorized data exposure.
// improvement: Verify that the authenticated user's ID matches the owner of the resource before returning data.

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
		json.NewEncoder(w).Encode(inv)
	}
}
