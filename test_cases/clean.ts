function safeAuth(req, res, next) {
    if (!req.headers.auth) {
        return res.status(401).send("No auth");
    }
    const user = parseToken(req.headers.auth);
    if (!user) {
        return res.status(401).send("Invalid token");
    }
    req.user = user;
    next();
}
