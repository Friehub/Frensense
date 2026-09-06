// [frensense]
// observation: An HTTP handler that performs sensitive operations (reading/writing data) has no authentication middleware and does not verify the caller's identity.
// impact: Any unauthenticated user can access sensitive endpoints to view, modify, or delete data without providing credentials.
// improvement: Add authentication middleware that validates a session token, JWT, or API key before the handler processes the request.

package main

import (
	"encoding/json"
	"net/http"
)

type Order struct {
	ID     int `json:"id"`
	UserID int `json:"user_id"`
	Total  float64 `json:"total"`
}

func listOrders(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		rows, _ := db.Query("SELECT id, user_id, total FROM orders")
		var orders []Order
		for rows.Next() {
			var o Order
			rows.Scan(&o.ID, &o.UserID, &o.Total)
			orders = append(orders, o)
		}
		json.NewEncoder(w).Encode(orders)
	}
}
