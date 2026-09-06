// [frensense]
// observation: exec() is called inside an async route handler with user-controlled input from req.body.target, enabling command injection across an async boundary.
// impact: An attacker can execute arbitrary OS commands despite the async wrapper, as exec() itself is not await-safe and runs synchronously in the shell.
// improvement: Use execFile() instead of exec(), and pass user input as array arguments rather than a shell string.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";
import { promisify } from "util";

const execAsync = promisify(exec);
const app = express();
app.use(express.json());

function extractPayload(req: express.Request): string {
    return req.body.target as string;
}

app.post("/api/process", async (req: express.Request, res: express.Response) => {
    const payload = extractPayload(req);
    try {
        const { stdout, stderr } = await execAsync(payload);
        res.json({ output: stdout, error: stderr });
    } catch (err: any) {
        res.status(500).json({ error: err.message });
    }
});
