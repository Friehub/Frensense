// SAFE: Redirect target is validated against an allowlist of known paths.
//       Unknown paths default to a safe landing page.

import express from "express";
import { Router } from "express";

const router = Router();
const ALLOWED_REDIRECTS = new Set(["/dashboard", "/profile", "/settings", "/help"]);

function getRedirectTarget(req: express.Request): string {
    const next = req.query.next as string | undefined;
    if (next && ALLOWED_REDIRECTS.has(next)) return next;
    return "/dashboard";
}

router.get("/login", (req: express.Request, res: express.Response) => {
    const target = getRedirectTarget(req);
    res.redirect(target);
});

router.post("/logout", (req: express.Request, res: express.Response) => {
    req.session?.destroy(() => {
        const next = req.body.returnTo as string | undefined;
        res.redirect(next && ALLOWED_REDIRECTS.has(next) ? next : "/");
    });
});

export default router;
