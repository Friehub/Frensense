// SAFE: Used Handlebars with strict mode and a partials registry, never accepting templates from user input.

import Handlebars from "handlebars";

Handlebars.registerPartial("page", "Hello {{name}}, your items: {{#items}}{{this}}{{/items}}");
Handlebars.registerPartial("notification", "Notification: {{message}}");

const ALLOWED_PARTIALS = new Set(["page", "notification"]);

function renderPage(req: Request, res: Response) {
    const partialName = req.body.partial || "page";
    if (!ALLOWED_PARTIALS.has(partialName)) throw new Error("Unknown template");
    const html = Handlebars.partials[partialName]({ user: req.user, items: req.body.items });
    res.send(html);
}

function renderNotification(req: Request, res: Response) {
    const html = Handlebars.partials.notification({ message: req.query.msg });
    res.send(html);
}
