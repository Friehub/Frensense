// [frensense]
// observation: User-controlled payload from req.body.payload is reflected in the HTML response without escaping.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// mutation: M2 different variable naming
// runtime_probe: xss

import express from "express";
import { Router } from "express";
import express from "express";

const router = Router();

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    res.send(`<html><body>Hello ${name}</body></html>`);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const payload = req.query.url as string;
    res.send(`<html><body>Hello ${name}</body></html>`);
});

export default router;
