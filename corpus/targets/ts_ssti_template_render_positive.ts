// [frensense]
// observation: User-controlled input is passed directly to a template rendering function (ejs.render/pug.render/etc), enabling Server-Side Template Injection (SSTI).
// impact: An attacker can inject template directives that execute arbitrary code on the server, leading to remote code execution or data exfiltration.
// improvement: Never pass unsanitized user input to template render functions. Use a static template with placeholders, or validate/sanitize user input before rendering.
// cwe: CWE-94
// cvss: 9.8
// owasp: A03:2021

import express from "express";

const app = express();

app.post("/render", (req: express.Request, res: express.Response) => {
    const template = req.body.template as string;
    const html = ejs.render(template, { data: req.body.data });
    res.send(html);
});