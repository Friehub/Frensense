// [frensense]
// observation: User input is concatenated into a GORM Raw SQL query, bypassing GORM's parameterized query protection and allowing SQL injection.
// impact: An attacker can craft input that alters the SQL query structure, enabling data exfiltration, modification, or deletion through GORM.
// improvement: Use GORM's parameter placeholders (?) in Raw queries or use the ORM query builder methods instead of raw SQL.

package main

import (
	"fmt"
	"net/http"

	"gorm.io/gorm"
)

func getUserByEmail(db *gorm.DB, w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	var user User
	db.Raw("SELECT * FROM users WHERE email = '" + email + "'").Scan(&user)
	fmt.Fprintf(w, "User: %s", user.Name)
}
