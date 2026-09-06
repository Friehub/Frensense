// [frensense]
// observation: User-controlled input is interpolated into a SQL query string passed to .query() without parameterization, enabling SQL injection.
// impact: An attacker can execute arbitrary SQL commands, exfiltrate data, or bypass authentication.
// improvement: Use parameterized queries with replacements or bind parameters instead of string interpolation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import express from "express";
import { Router } from "express";
import { Pool } from "pg"; const pool = new Pool();

const router = Router();

router.post("/api/login", (req: express.Request, res: express.Response, next: express.NextFunction) => {
    const email = req.body.email;
    pool.query(`SELECT * FROM users WHERE email = '${email}'`).then((result: any) => {
        res.json(result.rows);
    }).catch((err: Error) => {
        next(err);
    });
});

export default router;
