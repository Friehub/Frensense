// SAFE: Used a redirect map with predefined destinations, never accepting raw URLs from user input.

const REDIRECT_MAP: Record<string, string> = {
    "dashboard": "/dashboard",
    "settings": "/settings",
    "profile": "/profile",
    "home": "/",
    "login": "/login",
};

function redirectAfterLogin(req: Request, res: Response) {
    const dest = req.query.dest as string || "dashboard";
    const target = REDIRECT_MAP[dest] || "/dashboard";
    res.redirect(target);
}

function redirectAfterAction(req: Request, res: Response) {
    const dest = req.body.redirectTo || "home";
    const target = REDIRECT_MAP[dest] || "/";
    res.redirect(target);
}
