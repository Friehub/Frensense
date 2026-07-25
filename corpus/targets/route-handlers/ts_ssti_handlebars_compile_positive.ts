// [frensense]
// observation: User-controlled input is passed to Handlebars.compile() as a template string, allowing server-side template injection via Handlebars expressions.
// impact: An attacker can inject {{ }} expressions with built-in helpers to execute arbitrary code, read properties from the context, or perform prototype pollution.
// improvement: Use precompiled templates stored on disk and pass user input only as template data.
// cwe: CWE-1336
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import Handlebars from "handlebars";

function renderPage(req: Request, res: Response) {
    const userTemplate = req.body.template;
    const template = Handlebars.compile(userTemplate);
    const html = template({ user: req.user, items: req.body.items });
    res.send(html);
}

function renderNotification(req: Request, res: Response) {
    const notificationTemplate = req.query.tpl as string;
    const template = Handlebars.compile(notificationTemplate);
    const html = template({ message: req.query.msg });
    res.send(html);
}
