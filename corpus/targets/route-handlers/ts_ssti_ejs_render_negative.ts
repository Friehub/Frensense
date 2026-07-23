// SAFE: Used a static EJS template file and passed user input only as template data, never as the template itself.

import ejs from "ejs";
import { readFile } from "fs/promises";

async function renderReport(req: Request, res: Response) {
    const templateStr = await readFile("./templates/report.ejs", "utf-8");
    const data = { userName: req.body.userName, reportData: req.body.data };
    const html = await ejs.render(templateStr, data);
    res.send(html);
}

async function renderEmail(req: Request, res: Response) {
    const templateStr = await readFile("./templates/email.ejs", "utf-8");
    const html = ejs.render(templateStr, { user: req.user });
    res.send(html);
}
