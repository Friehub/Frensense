// [frensense]
// observation: User-controlled input reaches exec() through a promise chain.
// impact: An attacker can execute arbitrary OS commands.
// improvement: Use execFile with allowlist validation.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: cmdi

import { exec } from "child_process";
import express from "express";

const app = express();

app.post("/api/exec", (req: express.Request, res: express.Response) => {
    Promise.resolve(req.body.cmd as string)
        .then(cmd => new Promise<void>((resolve, reject) => {
            exec(cmd, (err, stdout) => {
                if (err) reject(err);
                else res.json({ result: stdout });
                resolve();
            });
        }))
        .catch(err => res.status(500).json({ error: err.message }));
});

export default app;
