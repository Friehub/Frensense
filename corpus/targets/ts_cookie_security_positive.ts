// Rule: TS_COOKIE_SECURITY
function handler(req: any, res: any) {
    res.cookie("session", "value"); // No security flags
}
