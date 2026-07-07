// [frensense]
// observation: The authentication middleware checks for a token, but falls back to assigning a default "guest" identity when the token is missing or invalid, rather than rejecting the request.
// impact: Endpoints relying on this middleware will inadvertently allow unauthenticated users to bypass security controls by assuming the guest identity.
// improvement: Return a 401 Unauthorized status (or throw an error) when authentication fails, instead of providing a fallback user object.
function authenticate(req: any, res: any, next: any) {
    const token = req.headers.authorization;
    let user;
    if (token) {
        user = verifyToken(token);
    } else {
        user = { role: "guest", id: 0 };
    }
    req.user = user;
    next();
}
