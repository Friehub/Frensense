// SAFE: If no valid token is present, an error is thrown immediately rather than falling through

function authenticate(req: any, res: any, next: any) {
    const token = req.headers.authorization;
    if (!token || !verifyToken(token)) {
        throw new Error("Unauthorized");
    }
    req.user = { id: token.sub, role: token.role };
    next();
}
