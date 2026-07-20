// SAFE: Wraps the handler with authentication middleware that validates the JWT token.

package main

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

var jwtSecret = []byte("your-secret-key")

func authMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		tokenStr := strings.TrimPrefix(authHeader, "Bearer ")
		token, err := jwt.Parse(tokenStr, func(token *jwt.Token) (interface{}, error) {
			return jwtSecret, nil
		})
		if err != nil || !token.Valid {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}
		next.ServeHTTP(w, r)
	})
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

func main() {
	mux := http.NewServeMux()
	mux.Handle("/orders", authMiddleware(listOrders(db)))
	http.ListenAndServe(":8080", mux)
}
