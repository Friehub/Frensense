// [frensense]
// observation: User-controlled input is passed to Mustache.render() as a template string, allowing injection via Mustache tags that reference context properties.
// impact: An attacker can access any property in the render context using {{ }} syntax, potentially leaking sensitive data or triggering side effects via getters.
// improvement: Use precompiled Mustache templates from disk and pass user input only as view data.
// cwe: CWE-1336
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import Mustache from "mustache";

function renderDocument(req: Request, res: Response) {
    const userTemplate = req.body.template;
    const html = Mustache.render(userTemplate, { user: req.user, doc: req.body.doc });
    res.send(html);
}

function renderPreview(req: Request, res: Response) {
    const previewTemplate = req.query.template as string;
    const html = Mustache.render(previewTemplate, { data: req.query });
    res.send(html);
}
