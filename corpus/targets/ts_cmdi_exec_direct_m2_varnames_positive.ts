// [frensense]
// observation: User-controlled payload from req.body.payload is passed to exec() without sanitization.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// mutation: M2 different variable naming
// runtime_probe: payloadi

import express from "express";
import { Router } from "express";
import { exec } from "child_process";

const router = Router();

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    exec(payload, (err, stdout) => { res.json({ result: stdout }); });
});

router.post("/api/admin", (req: express.Request, res: express.Response) => {
    const payload = req.query.url as string;
    exec(payload, (err, stdout) => { res.json({ result: stdout }); });
});

export default router;
