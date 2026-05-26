// Rule: TS_COOKIE_SECURITY (negative — no rule expected)
function handler(req: any, res: any) {
    res.cookie("session", "value", { httpOnly: true });
}
