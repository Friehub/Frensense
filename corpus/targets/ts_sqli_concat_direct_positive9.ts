// [frensense]
// observation: User-controlled input from req.body.target is concatenated into a SQL query without parameterization.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import express from "express";

const app = express();

app.all("/exec", (req: express.Request, res: express.Response) => {
    const command = req.query.cmd as string || req.body.cmd as string;
    exec(command, (err, stdout) => {
        res.json({ output: stdout });
    });
});

export default app;
