// [frensense]
// observation: execSync() is called with user-controlled input from req.body.command, executing arbitrary shell commands synchronously and blocking the event loop.
// impact: An attacker can execute arbitrary OS commands and cause a denial-of-service by blocking the event loop with a long-running command.
// improvement: Use execFile() with array arguments, or use async exec() with proper error handling. Never use execSync() with user input.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { execSync } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

function readInput(req: express.Request): string {
    return req.body.command as string;
}

app.post("/api/exec", (req: express.Request, res: express.Response) => {
    const cmd = readInput(req);
    try {
        const output = execSync(cmd, { encoding: "utf8" });
        res.json({ output });
    } catch (err: any) {
        res.status(500).json({ error: err.stderr || err.message });
    }
});
