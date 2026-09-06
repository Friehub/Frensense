// Scenario 1: Misleading Comments (Fix 1 verification)
// This function name starts with 'validate', so it triggers TS_CSA_VALIDATE_UNCONDITIONAL.
// It lacks an actual 'return false' or 'throw', but it HAS them in comments.
// Frensense v0.2.0 (Regex) would say: "I see 'return false', it's fine."
// Frensense v0.3.0 (Reachability) should say: "The comment doesn't count. CRITICAL."
fn validate_user_profile(profile: &Profile) {
    println!("Checking profile...");
    // TODO: return false if profile is invalid;
    /* throw Error("invalid"); */
}

// Scenario 2: Delegation (Fix 2 verification)
// This function doesn't have 'return false' or 'throw' either.
// But it calls 'safeParse', which is in our delegation whitelist.
// Frensense v0.3.0 should suppress this finding.
fn validate_request(req: &Request) {
    schema.safeParse(req);
}
