// [frensense]
// observation: User-controlled input from req.query.url or req.body.target is
//              passed to an HTTP client without host validation.
// impact: An attacker can reach internal services or cloud metadata endpoints.
// improvement: Validate the URL against an allowlist of permitted hosts.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import express from "express";
import { Router } from "express";
import { Pool } from "pg";
const pool = new Pool();

const router = Router();

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`);
    res.json(result.rows);
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    const url = req.query.url as string;
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`);
    res.json(result.rows);
});

export default router;
