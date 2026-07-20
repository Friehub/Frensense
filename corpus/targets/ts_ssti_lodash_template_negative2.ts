// SAFE: Used _.template() with a fixed, pre-compiled template; user input is only passed as data.

import _ from "lodash";

const GREETING_TPL = _.template("Hello <%= name %>, your role is <%= role %>");
const MESSAGE_TPL = _.template("User: <%= user.name %>, Data: <%= JSON.stringify(data) %>");

function renderGreeting(req: Request, res: Response) {
    const html = GREETING_TPL({ name: req.user.name, role: req.user.role });
    res.send(html);
}

function renderMessage(req: Request, res: Response) {
    const result = MESSAGE_TPL({ user: req.user, data: req.body });
    res.json({ message: result });
}
