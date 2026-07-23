// [frensense]
// observation: User-controlled input is passed to nunjucks.renderString() as a template, allowing server-side template injection via Nunjucks syntax.
// impact: An attacker can inject {{ }} expressions to access the global context, call template functions, or execute arbitrary code through Nunjucks built-in features.
// improvement: Use static template files with nunjucks.render() and pass user input only as context data.

import nunjucks from "nunjucks";

nunjucks.configure("./views", { autoescape: true });

function renderInvoice(req: Request, res: Response) {
    const templateStr = req.body.template;
    const html = nunjucks.renderString(templateStr, { invoice: req.body.invoice, user: req.user });
    res.send(html);
}

function renderCustomPage(req: Request, res: Response) {
    const tpl = req.query.tpl as string;
    const html = nunjucks.renderString(tpl, { session: req.session });
    res.send(html);
}
