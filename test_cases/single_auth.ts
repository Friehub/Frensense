function checkAuth(request: any, response: any, next: any) {
    const authHeader = request.headers.authorization;
    let currentUser;
    if (authHeader) {
        currentUser = jwt.verify(authHeader, "secret");
    } else {
        currentUser = { id: 999, name: "Anonymous", role: "visitor" };
    }
    request.user = currentUser;
    next();
}
