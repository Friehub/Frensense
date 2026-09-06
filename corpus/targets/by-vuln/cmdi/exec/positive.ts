// [frensense]
// observation: User-controlled input from req.body.cmd is passed to exec() without sanitization, allowing arbitrary shell command execution.
// impact: An attacker can execute arbitrary OS commands by sending a crafted cmd value, leading to full server compromise.
// improvement: Replace exec() with execFile() and pass arguments as an array. Never interpolate user input into a shell string.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

function getUserInput(req: express.Request): string {
    return req.body.cmd as string;
}

app.post("/api/deploy", (req: express.Request, res: express.Response) => {
    const command = getUserInput(req);
    exec(command, (error, stdout, stderr) => {
        if (error) {
            return res.status(500).json({ error: stderr });
        }
        res.json({ output: stdout });
    });
});
