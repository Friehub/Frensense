// SAFE: Validated redirect URL against an allowlist of permitted domains before redirecting.

const ALLOWED_REDIRECT_DOMAINS = new Set([
    "example.com",
    "app.example.com",
    "www.example.com",
]);

function isSafeRedirect(url: string): boolean {
    try {
        const parsed = new URL(url, "https://example.com");
        return ALLOWED_REDIRECT_DOMAINS.has(parsed.hostname);
    } catch {
        return false;
    }
}

function redirectAfterLogin(req: Request, res: Response) {
    const redirectUrl = req.query.redirect as string;
    if (isSafeRedirect(redirectUrl)) {
        res.redirect(redirectUrl);
    } else {
        res.redirect("/dashboard");
    }
}

function redirectAfterAction(req: Request, res: Response) {
    const returnTo = req.body.returnUrl;
    if (isSafeRedirect(returnTo)) {
        res.redirect(returnTo);
    } else {
        res.redirect("/");
    }
}
