// [frensense]
// observation: User-controlled input is passed to Set-Cookie or Location headers without CRLF sanitization, allowing HTTP response splitting.
// impact: An attacker can inject CRLF (%0d%0a) sequences to split the HTTP response, inject arbitrary headers, set arbitrary cookies, or perform cache poisoning.
// improvement: Strip CRLF characters from all user input before including it in response headers.

function setUserCookie(req: Request, res: Response) {
    const username = req.body.username;
    res.setHeader("Set-Cookie", `username=${username}; Path=/`);
    res.json({ success: true });
}

function redirectWithMessage(req: Request, res: Response) {
    const message = req.query.msg as string;
    const dest = "/";
    res.setHeader("Location", `${dest}?message=${message}`);
    res.status(302).end();
}

function customCookie(req: Request, res: Response) {
    const cookieName = req.body.cookieName;
    const cookieValue = req.body.cookieValue;
    res.setHeader("Set-Cookie", `${cookieName}=${cookieValue}; HttpOnly; Secure`);
    res.json({ success: true });
}
