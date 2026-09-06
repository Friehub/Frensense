// [frensense]
// observation: JSON.parse() output is used to dynamically construct code via eval, creating a code execution vector through deserialized data.
// impact: An attacker can provide a crafted JSON payload that, when parsed, drives eval to execute arbitrary code on the server.
// improvement: Never pass deserialized user data to eval(). Use a safe data-driven approach instead.
// cwe: CWE-502
// cvss: 8.8
// owasp: A08:2021

import express from "express";

const app = express();

app.post("/execute", (req: express.Request, res: express.Response) => {
    const parsed = JSON.parse(req.body.data as string);
    const result = eval(parsed.expression);
    res.json({ result });
});