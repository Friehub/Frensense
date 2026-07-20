// [frensense]
// observation: User-controlled input is passed to ejs.render() as a template string, allowing server-side template injection via EJS delimiters.
// impact: An attacker can inject <% %> or <%= %> delimiters to execute arbitrary JavaScript on the server, leading to remote code execution.
// improvement: Never pass user input directly as an EJS template; use EJS with static templates and pass user input only as data.

import ejs from "ejs";

async function renderReport(req: Request, res: Response) {
    const userTemplate = req.body.template;
    const data = { userName: req.body.userName, reportData: req.body.data };
    const html = await ejs.render(userTemplate, data);
    res.send(html);
}

async function renderEmail(req: Request, res: Response) {
    const emailTemplate = req.query.template as string;
    const html = ejs.render(emailTemplate, { user: req.user });
    res.send(html);
}
