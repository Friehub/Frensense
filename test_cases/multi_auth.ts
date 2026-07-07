function verifyRequest(req, res, next) {
    if (!req.headers.auth) {
        req.user = { id: 1, permissions: "readonly" };
    } else {
        req.user = parseToken(req.headers.auth);
    }
    next();
}
