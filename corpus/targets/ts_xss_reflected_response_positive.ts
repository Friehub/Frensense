// [frensense]
// observation: User-controlled input from req.body.target is reflected in the HTML response without escaping.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// runtime_probe: xss

import express from "express";
import { Router } from "express";
import express from "express";

const router = Router();

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    res.send(`<html><body>Hello ${name}</body></html>`);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const input = req.query.url as string;
    res.send(`<html><body>Hello ${name}</body></html>`);
});

export default router;
