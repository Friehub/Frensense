// [frensense]
// observation: User-controlled input from req.body.target is passed to exec() without sanitization.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: cmdi

import express from "express";
import { Router } from "express";
import { exec } from "child_process";

const router = Router();

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    exec(cmd, (err, stdout) => { res.json({ result: stdout }); });
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const input = req.query.url as string;
    exec(cmd, (err, stdout) => { res.json({ result: stdout }); });
});

export default router;
