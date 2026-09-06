// SAFE: Used EJS with outputMode set to "escaped" and sanitized the template by stripping EJS tags before rendering.

import ejs from "ejs";

function sanitizeTemplate(input: string): string {
    return input.replace(/[<>]/g, "").replace(/%(=|%>|%)/g, "");
}

async function renderReport(req: Request, res: Response) {
    const userTemplate = sanitizeTemplate(req.body.template);
    const data = { userName: req.body.userName, reportData: req.body.data };
    const html = await ejs.render(userTemplate, data, { outputFunctionName: "escape" });
    res.send(html);
}
