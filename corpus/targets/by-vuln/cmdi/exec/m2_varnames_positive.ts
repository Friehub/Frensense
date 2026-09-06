// [frensense]
// observation: User-controlled input is destructured from req.body, renamed through intermediate assignments, and reaches exec() without sanitization.
// impact: The variable renaming obfuscates the taint path but does not prevent injection — same exploitation as direct exec().
// improvement: Avoid renaming tainted variables without sanitizing. Apply validation at the earliest point of input receipt.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";

const router = express.Router();

router.post("/api/convert", (req: express.Request, res: express.Response) => {
    const { source: input } = req.body as { source: string };
    const command = input;
    const cmd = command;
    exec(cmd, (err, stdout) => {
        res.json({ data: stdout });
    });
});

export default router;
