// [frensense]
// observation: The redirect destination is taken directly from req.query.next
//              without validating that it points to the same origin.
// impact: An attacker can craft a link like /login?next=https://evil.com that
//         redirects users to a phishing site after a successful login.
// improvement: Validate that `next` is a relative path (starts with /) and does
//              not contain a protocol or host component before redirecting.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// runtime_probe: redirect

import express from "express";
import { Router } from "express";

const router = Router();

function getRedirectTarget(req: express.Request): string {
    return req.query.next as string ?? "/dashboard";
}

router.get("/login", (req: express.Request, res: express.Response) => {
    // ... auth logic ...
    const target = getRedirectTarget(req);
    res.redirect(target);
});

router.post("/logout", (req: express.Request, res: express.Response) => {
    req.session?.destroy(() => {
        const next = req.body.returnTo as string;
        res.redirect(next ?? "/");
    });
});

export default router;
