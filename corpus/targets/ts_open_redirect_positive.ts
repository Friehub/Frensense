// Rule: TS_OPEN_REDIRECT
function handleLogin(req: any, res: any) {
    const userId = req.session.userId;
    if (!userId) {
        return res.status(401).json({ error: "Unauthorized" });
    }

    const redirectTo = req.query.next || "/dashboard";
    const dbUser = db.findUser(userId);

    if (!dbUser || dbUser.banned) {
        return res.status(403).json({ error: "Forbidden" });
    }

    res.redirect(redirectTo);
}
