// [frensense]
// observation: User-controlled URL is passed to the Location header for redirects, allowing open redirect and header injection via crafted URLs.
// impact: An attacker can inject CRLF sequences in the URL to perform HTTP response splitting, or redirect users to phishing sites (open redirect).
// improvement: Validate the redirect URL against an allowlist of permitted domains and strip CRLF characters.

function redirectAfterLogin(req: Request, res: Response) {
    const redirectUrl = req.query.redirect as string;
    res.status(302);
    res.setHeader("Location", redirectUrl);
    res.end();
}

function redirectAfterAction(req: Request, res: Response) {
    const returnTo = req.body.returnUrl;
    res.redirect(returnTo);
}
