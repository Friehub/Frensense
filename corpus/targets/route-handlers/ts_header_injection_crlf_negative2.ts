// SAFE: Used express-session for cookie management instead of raw Set-Cookie headers, and validated URLs before redirecting.

import session from "express-session";

function setUserCookie(req: Request, res: Response) {
    req.session.username = req.body.username;
    res.json({ success: true });
}

function redirectWithMessage(req: Request, res: Response) {
    const message = req.query.msg as string;
    const safeMessage = encodeURIComponent(message.replace(/[\r\n]/g, " "));
    res.redirect(`/?message=${safeMessage}`);
}

function customCookie(req: Request, res: Response) {
    req.session.customData = {
        name: req.body.cookieName,
        value: req.body.cookieValue,
    };
    res.json({ success: true });
}
