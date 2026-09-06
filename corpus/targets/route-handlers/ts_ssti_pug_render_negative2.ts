// SAFE: Used Pug with a restricted locals scope — only allows specific variables in the template context.

import pug from "pug";

const profileTemplate = pug.compile("h1 Welcome #{name}\np Profile of #{name}", { basedir: "./views" });

function renderProfile(req: Request, res: Response) {
    const safeLocals = {
        name: req.user.name,
        email: req.user.email,
    };
    const html = profileTemplate(safeLocals);
    res.send(html);
}

function renderWidget(req: Request, res: Response) {
    const safeLocals = {
        title: "Widget",
        items: [],
    };
    const html = pug.render("h1= title\nul\n each item in items\n  li= item", safeLocals);
    res.send(html);
}
