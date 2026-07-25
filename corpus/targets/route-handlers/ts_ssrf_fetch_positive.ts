// [frensense]
// observation: User-controlled input from req.query.url or req.body.target is
//              passed to an HTTP client without host validation.
// impact: An attacker can reach internal services or cloud metadata endpoints.
// improvement: Validate the URL against an allowlist of permitted hosts.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// runtime_probe: ssrf

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    const url = req.query.url as string;
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
});

export default router;
