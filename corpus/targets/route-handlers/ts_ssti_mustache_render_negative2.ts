// SAFE: Used Mustache with partials system and a limited context, only exposing specific variables to the template.

import Mustache from "mustache";

Mustache.parse("{{>userInfo}}");
Mustache.parse("{{>docMeta}}");

function renderDocument(req: Request, res: Response) {
    const view = {
        userName: req.user.name,
        userEmail: req.user.email,
        docTitle: req.body.doc.title,
        docBody: req.body.doc.body,
    };
    const html = Mustache.render("{{>userInfo}}\n{{>docMeta}}", view, {
        userInfo: "User: {{userName}} ({{userEmail}})",
        docMeta: "Document: {{docTitle}}\n{{docBody}}",
    });
    res.send(html);
}

function renderPreview(req: Request, res: Response) {
    const view = {
        value1: req.query.param1,
        value2: req.query.param2,
    };
    const html = Mustache.render("Preview: {{value1}} - {{value2}}", view);
    res.send(html);
}
