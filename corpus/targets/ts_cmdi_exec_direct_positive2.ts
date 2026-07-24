// [frensense]
// observation: User-controlled input is passed to exec() via array destructuring.
// impact: An attacker can execute arbitrary OS commands.
// improvement: Use execFile with array arguments.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();

router.post("/api/cmd", (req: express.Request, res: express.Response) => {
    const [cmd] = [req.body.cmd as string];
    exec(cmd, (err, stdout) => {
        res.json({ result: stdout });
    });
});

export default router;
