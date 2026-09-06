// SAFE: Redirect target is validated to be a relative path starting with /
//       before being used. Absolute URLs and protocol-relative URLs are rejected.

import express from "express";
import { Router } from "express";

const router = Router();
const SAFE_PATH_RE = /^\/(?:[a-zA-Z0-9_\-\.\/]+)?$/;

function getRedirectTarget(req: express.Request): string {
    const next = req.query.next as string | undefined;
    if (next && SAFE_PATH_RE.test(next)) return next;
    return "/dashboard";
}

router.get("/login", (req: express.Request, res: express.Response) => {
    const target = getRedirectTarget(req);
    res.redirect(target);
});

router.post("/logout", (req: express.Request, res: express.Response) => {
    req.session?.destroy(() => {
        const next = req.body.returnTo as string | undefined;
        if (next && SAFE_PATH_RE.test(next)) {
            res.redirect(next);
        } else {
            res.redirect("/");
        }
    });
});

export default router;
