// [frensense]
// observation: math/rand (a pseudo-random number generator) is used to generate security-sensitive values like password reset tokens or session IDs, which are predictable if the seed is known.
// impact: An attacker can predict or brute-force the token by seeding their own math/rand with the same time-based seed and enumerating values, enabling account takeover.
// improvement: Use crypto/rand for security-sensitive random values, which provides cryptographically secure random bytes.

package main

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func requestPasswordReset(w http.ResponseWriter, r *http.Request) {
	email := r.FormValue("email")
	rand.Seed(time.Now().UnixNano())
	token := fmt.Sprintf("%08x", rand.Uint32())
	err := saveResetToken(email, token)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "Reset link sent to %s with token %s", email, token)
}
