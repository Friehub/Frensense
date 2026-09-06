// [frensense]
// observation: User-controlled input is passed to _.template() and invoked with data, allowing server-side template injection via Lodash template delimiters.
// impact: An attacker can inject <%= %> or <%- %> delimiters to execute arbitrary JavaScript through Lodash's template compilation, leading to RCE.
// improvement: Avoid _.template() with user input; use a safe string interpolation or pass user data through a static template.
// cwe: CWE-1336
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import _ from "lodash";

function renderGreeting(req: Request, res: Response) {
    const tpl = req.body.greetingTemplate;
    const compiled = _.template(tpl);
    const html = compiled({ name: req.user.name, role: req.user.role });
    res.send(html);
}

function renderMessage(req: Request, res: Response) {
    const msgTemplate = req.query.format as string;
    const compiled = _.template(msgTemplate);
    const result = compiled({ user: req.user, data: req.body });
    res.json({ message: result });
}
