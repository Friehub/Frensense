// [frensense]
// mutation: M1 async/await variant
// observation: User-controlled input from req.body.target is passed to fetch() without host validation.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// runtime_probe: ssrf

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const input = req.query.url as string;
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

export default router;
