// Rule: TS_OPEN_REDIRECT (negative — no rule expected)
function handler(req: any, res: any) {
    res.redirect("https://example.com"); // Hardcoded URL — safe
}
