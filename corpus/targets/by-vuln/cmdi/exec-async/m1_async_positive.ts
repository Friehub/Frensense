// [frensense]
// observation: execSync is called inside an async route handler with req.body.script, executing shell commands synchronously despite the async context.
// impact: An attacker can execute arbitrary commands and the synchronous call blocks the entire event loop, amplifying the denial-of-service impact.
// improvement: Replace execSync with execFile and use proper async/await patterns.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { execSync } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

function getScriptPath(req: express.Request): string {
    return req.body.script as string;
}

app.post("/api/compile", async (req: express.Request, res: express.Response) => {
    const script = getScriptPath(req);
    try {
        const result = execSync(`/usr/bin/compile ${script}`, {
            encoding: "utf8",
            timeout: 10000,
        });
        res.json({ compiled: result });
    } catch (err: any) {
        res.status(500).json({ error: err.message });
    }
});
