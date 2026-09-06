// [frensense]
// observation: User-controlled payload from req.body.payload is passed to fetch() without host validation.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// mutation: M2 different variable naming
// runtime_probe: ssrf

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const payload = req.query.url as string;
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

export default router;
