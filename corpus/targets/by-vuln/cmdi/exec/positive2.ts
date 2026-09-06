// [frensense]
// observation: User-controlled input from req.query.script is passed to exec() via a helper function, allowing arbitrary command execution.
// impact: An attacker can execute arbitrary OS commands by crafting a script query parameter, bypassing intended command restrictions.
// improvement: Validate the script name against an allowlist of known scripts before executing.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";

const router = express.Router();

function resolveCommand(input: string): string {
    return `/usr/local/bin/${input}`;
}

router.get("/api/run", (req: express.Request, res: express.Response) => {
    const script = req.query.script as string;
    const cmd = resolveCommand(script);
    exec(cmd, (err, stdout) => {
        res.json({ result: stdout });
    });
});

export default router;
