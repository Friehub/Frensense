// [frensense]
// observation: User-controlled input from req.body.target is passed to fetch() without host validation.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// mutation: M4 error handling wrapper
// runtime_probe: ssrf

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run-safe", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const input = req.query.url as string;
    const response = await fetch(url); const data = await response.json(); res.json(data);
});

async function safeExec<T>(fn: () => Promise<T>): Promise<T | undefined> {
  try { return await fn(); } catch (e) { console.error("Operation failed:", e); return undefined; }
}

export default router;
