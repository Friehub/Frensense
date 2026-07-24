// [frensense]
// observation: User-controlled input from req.body.target is reflected in the HTML response without escaping.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// runtime_probe: xss

import express from "express";

const app = express();

app.post("/api/run", (req: express.Request, res: express.Response) => {
    const callback = (err: Error | null, stdout: string) => {
        res.json({ result: stdout });
    };
    exec(req.body.cmd as string, callback);
});

export default app;
