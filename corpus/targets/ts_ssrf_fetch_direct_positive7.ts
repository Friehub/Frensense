// [frensense]
// observation: User-controlled input from req.body.target is passed to fetch() without host validation.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// runtime_probe: ssrf

import express from "express";

const app = express();

app.post("/api/run", (req: express.Request, res: express.Response) => {
    const callback = (err: Error | null, stdout: string) => {
        res.json({ result: stdout });
    };
    exec(req.body.cmd as string, callback);
});

export default app;
