// [frensense]
// observation: User-controlled input traverses multiple variable assignments before being interpolated into HTML response without escaping.
// impact: An attacker can inject arbitrary HTML/JavaScript via multi-hop assignment, enabling XSS.
// improvement: HTML-escape user input regardless of how many assignment hops occur.

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    const a = req.query.q as string;
    const b = a;
    res.send(`<html><body><h1>Search results for: ${b}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const raw = req.query.name as string;
    const name = raw;
    res.send(`<p>Welcome, ${name}!</p>`);
}
