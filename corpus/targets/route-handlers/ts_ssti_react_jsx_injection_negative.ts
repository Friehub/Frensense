// SAFE: Removed eval() and used React.createElement directly with validated tag names and props.

import React from "react";
import { renderToString } from "react-dom/server";

const ALLOWED_TAGS = new Set(["div", "span", "p", "h1", "h2", "h3", "ul", "li", "table", "tr", "td", "th"]);

function renderCustomComponent(req: Request, res: Response) {
    const content = req.body.content;
    const html = renderToString(React.createElement("div", { className: "content" }, content));
    res.send(html);
}

function renderDynamicElement(req: Request, res: Response) {
    const tagName = req.query.tag as string;
    if (!ALLOWED_TAGS.has(tagName)) throw new Error("Invalid tag");
    const content = req.query.content as string;
    const element = React.createElement(tagName, { className: "dynamic" }, content);
    const html = renderToString(element);
    res.send(html);
}
