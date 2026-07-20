// SAFE: Used precompiled Pug templates from disk; user input is only passed as template locals.

import pug from "pug";

const profileTemplate = pug.compileFile("./templates/profile.pug");
const widgetTemplate = pug.compileFile("./templates/widget.pug");

function renderProfile(req: Request, res: Response) {
    const html = profileTemplate({ user: req.user });
    res.send(html);
}

function renderWidget(req: Request, res: Response) {
    const html = widgetTemplate({ data: req.query });
    res.send(html);
}
