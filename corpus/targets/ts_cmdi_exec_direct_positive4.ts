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

const router = Router();

router.post("/api/conditional", (req: express.Request, res: express.Response) => {
    const cmd = req.body.cmd as string;
    if (req.body.enabled) {
        exec(cmd, (err, stdout) => {
            res.json({ result: stdout });
        });
    }
});

export default router;
