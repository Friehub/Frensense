// SAFE: Used a component registry with fixed render functions, never accepting code from user input.

import React from "react";
import { renderToString } from "react-dom/server";

const COMPONENT_REGISTRY: Record<string, (props: any) => React.ReactElement> = {
    greeting: (props) => React.createElement("h1", null, `Hello ${props.name}`),
    list: (props) => React.createElement("ul", null, props.items.map((i: string) => React.createElement("li", null, i))),
    card: (props) => React.createElement("div", null,
        React.createElement("h2", null, props.title),
        React.createElement("p", null, props.body),
    ),
};

function renderCustomComponent(req: Request, res: Response) {
    const componentName = req.body.componentName;
    const factory = COMPONENT_REGISTRY[componentName];
    if (!factory) throw new Error("Unknown component");
    const element = factory(req.body.props || {});
    const html = renderToString(element);
    res.send(html);
}

function renderDynamicElement(req: Request, res: Response) {
    const element = React.createElement("div", null,
        React.createElement("p", null, req.query.content as string),
    );
    const html = renderToString(element);
    res.send(html);
}
