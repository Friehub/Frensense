// [frensense]
// observation: User-controlled payload from req.body.payload is concatenated into a SQL query without parameterization.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// mutation: M2 different variable naming
// runtime_probe: sqli

import express from "express";
import { Router } from "express";
import { Pool } from "pg"; const pool = new Pool();

const router = Router();

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`); res.json(result.rows);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const payload = req.query.url as string;
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`); res.json(result.rows);
});

export default router;
