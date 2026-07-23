// [frensense]
// observation: User-controlled input is passed to eval() or new Function() within JSX rendering logic, allowing code injection through React element construction.
// impact: An attacker can inject arbitrary JavaScript execution on the server by crafting input that reaches eval() inside React.createElement or JSX compilation.
// improvement: Never use eval() with user input; use React.createElement directly with validated props.

import React from "react";
import { renderToString } from "react-dom/server";

function renderCustomComponent(req: Request, res: Response) {
    const userCode = req.body.componentCode;
    const Component = eval(`(props) => React.createElement("div", null, ${userCode})`);
    const html = renderToString(React.createElement(Component, { data: req.body.data }));
    res.send(html);
}

function renderDynamicElement(req: Request, res: Response) {
    const tagName = req.query.tag as string;
    const content = req.query.content as string;
    const element = eval(`React.createElement("${tagName}", null, "${content}")`);
    const html = renderToString(element);
    res.send(html);
}
