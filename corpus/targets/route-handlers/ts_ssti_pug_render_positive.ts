// [frensense]
// observation: User-controlled input is passed to pug.render() as a template string, allowing server-side template injection via Pug syntax.
// impact: An attacker can inject Pug mixins or JavaScript interpolation (#{}) to execute arbitrary code on the server, leading to RCE.
// improvement: Use precompiled Pug template files and pass user input only as template locals.
// cwe: CWE-1336
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import pug from "pug";

async function renderProfile(req: Request, res: Response) {
    const userTemplate = req.body.template;
    const html = await pug.render(userTemplate, { user: req.user });
    res.send(html);
}

function renderWidget(req: Request, res: Response) {
    const widgetTemplate = req.query.widgetTpl as string;
    const html = pug.render(widgetTemplate, { data: req.query });
    res.send(html);
}
