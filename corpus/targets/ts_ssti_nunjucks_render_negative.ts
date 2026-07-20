// SAFE: Used nunjucks.render() with static template files instead of renderString with user-supplied templates.

import nunjucks from "nunjucks";

nunjucks.configure("./views", { autoescape: true });

function renderInvoice(req: Request, res: Response) {
    const html = nunjucks.render("invoice.html", { invoice: req.body.invoice, user: req.user });
    res.send(html);
}

function renderCustomPage(req: Request, res: Response) {
    const html = nunjucks.render("custom.html", { session: req.session });
    res.send(html);
}
