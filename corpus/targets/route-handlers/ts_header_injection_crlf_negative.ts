// SAFE: Stripped CRLF sequences from all user input before including in response headers.

function sanitizeHeader(input: string): string {
    return input.replace(/[\r\n%0d%0a]/gi, "").replace(/\0/g, "");
}

function setUserCookie(req: Request, res: Response) {
    const username = sanitizeHeader(req.body.username);
    res.setHeader("Set-Cookie", `username=${username}; Path=/`);
    res.json({ success: true });
}

function redirectWithMessage(req: Request, res: Response) {
    const message = sanitizeHeader(req.query.msg as string);
    const dest = "/";
    res.setHeader("Location", `${dest}?message=${message}`);
    res.status(302).end();
}

function customCookie(req: Request, res: Response) {
    const cookieName = sanitizeHeader(req.body.cookieName);
    const cookieValue = sanitizeHeader(req.body.cookieValue);
    res.setHeader("Set-Cookie", `${cookieName}=${cookieValue}; HttpOnly; Secure`);
    res.json({ success: true });
}
