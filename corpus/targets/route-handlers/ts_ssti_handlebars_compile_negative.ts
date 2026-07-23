// SAFE: Used precompiled templates from the filesystem with user input passed only as data.

import Handlebars from "handlebars";
import { readFileSync } from "fs";

const pageTemplate = Handlebars.compile(readFileSync("./templates/page.hbs", "utf-8"));
const notificationTemplate = Handlebars.compile(readFileSync("./templates/notification.hbs", "utf-8"));

function renderPage(req: Request, res: Response) {
    const html = pageTemplate({ user: req.user, items: req.body.items });
    res.send(html);
}

function renderNotification(req: Request, res: Response) {
    const html = notificationTemplate({ message: req.query.msg });
    res.send(html);
}
