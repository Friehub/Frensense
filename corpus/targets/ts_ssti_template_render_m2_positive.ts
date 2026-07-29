// [frensense]
// observation: User-controlled input is passed directly to pug.render, allowing template injection via Pug syntax.
// impact: An attacker can inject Pug template directives that execute arbitrary JavaScript on the server.
// improvement: Never render user-supplied template strings. Compile templates statically and pass user data as context variables only.
// cwe: CWE-94
// cvss: 9.8
// owasp: A03:2021

import express from "express";

const app = express();

app.post("/page", (req: express.Request, res: express.Response) => {
    const html = pug.render(req.body.layout as string, { user: req.user });
    res.send(html);
});