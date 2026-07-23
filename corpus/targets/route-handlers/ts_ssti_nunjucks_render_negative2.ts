// SAFE: Used nunjucks.render() with a map of allowed templates, mapping user-provided names to static template files.

import nunjucks from "nunjucks";

nunjucks.configure("./views", { autoescape: true });

const ALLOWED_TEMPLATES: Record<string, string> = {
    invoice: "invoice.html",
    receipt: "receipt.html",
    statement: "statement.html",
};

function renderInvoice(req: Request, res: Response) {
    const tplName = req.body.templateName || "invoice";
    const tplFile = ALLOWED_TEMPLATES[tplName];
    if (!tplFile) throw new Error("Unknown template");
    const html = nunjucks.render(tplFile, { invoice: req.body.invoice, user: req.user });
    res.send(html);
}

function renderCustomPage(req: Request, res: Response) {
    const html = nunjucks.render("custom.html", { session: req.session });
    res.send(html);
}
