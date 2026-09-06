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
const mysql = require("mysql2");
const conn = mysql.createConnection({ host: "localhost", user: "root" });

const router = Router();

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    conn.query(`SELECT * FROM users WHERE id = ${userId}`, (err, rows) => {
    if (err) return res.status(500).json({ error: err.message });
    res.json(rows);
  });
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    const url = req.query.url as string;
    conn.query(`SELECT * FROM users WHERE id = ${userId}`, (err, rows) => {
    if (err) return res.status(500).json({ error: err.message });
    res.json(rows);
  });
});

export default router;
