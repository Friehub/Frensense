// Rule: TS_SQL_INJECTION (negative — no rule expected)
function getUser() {
    db.query("SELECT 1"); // No user input — safe
}
