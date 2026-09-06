// [frensense]
// observation: User-controlled input from req.body.target is concatenated into a SQL query without parameterization.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// mutation: M4 error handling wrapper
// runtime_probe: sqli

import express from "express";
import { Router } from "express";
import { Pool } from "pg"; const pool = new Pool();

const router = Router();

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run-safe", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`); res.json(result.rows);
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const input = req.query.url as string;
    const result = await pool.query(`SELECT * FROM users WHERE id = ${userId}`); res.json(result.rows);
});

async function safeExec<T>(fn: () => Promise<T>): Promise<T | undefined> {
  try { return await fn(); } catch (e) { console.error("Operation failed:", e); return undefined; }
}

export default router;
