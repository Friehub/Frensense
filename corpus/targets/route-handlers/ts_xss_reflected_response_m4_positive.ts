// [frensense]
// observation: User-controlled input is passed through a helper function that returns the value without escaping before embedding in HTML response.
// impact: An attacker can inject arbitrary HTML/JavaScript through the helper function, enabling XSS.
// improvement: Apply HTML escaping inside the helper function before returning.

import express from "express";

function getQuery(req: express.Request): string {
    return req.query.q as string;
}

function getName(req: express.Request): string {
    return req.query.name as string;
}

export function searchHandler(req: express.Request, res: express.Response) {
    const query = getQuery(req);
    res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = getName(req);
    res.send(`<p>Welcome, ${name}!</p>`);
}
