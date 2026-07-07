function authenticate(req: any, res: any, next: any) {
    const token = req.headers.authorization;
    if (!token) {
        return res.status(401).send("Unauthorized");
    }
    const user = verifyToken(token);
    if (!user) {
        return res.status(401).send("Unauthorized");
    }
    req.user = user;
    next();
}
