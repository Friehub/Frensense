// [frensense]
// observation: The JWT token is parsed using jwt.Parse (or similar) without calling the key validation function, meaning the token signature is not cryptographically verified.
// impact: An attacker can forge arbitrary JWT tokens with any claims (e.g., admin role) because the signature is never checked, leading to authentication bypass.
// improvement: Use jwt.ParseWithClaims or jwt.Parse and pass a key validation callback that verifies the token was signed with the correct secret or public key.

package main

import (
	"fmt"
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func authMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		tokenStr := strings.TrimPrefix(authHeader, "Bearer ")
		token, _ := jwt.Parse(tokenStr, nil)
		if token == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}
		claims := token.Claims.(jwt.MapClaims)
		role := claims["role"].(string)
		if role != "admin" {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}
