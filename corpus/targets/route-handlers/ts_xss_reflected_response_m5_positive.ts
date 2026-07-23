// [frensense]
// observation: User-controlled input is interpolated via template literal into the HTML response body without escaping.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: HTML-escape all user input before template literal interpolation in responses.

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    const query = req.query.q as string;
    res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    res.send(`<p>Welcome, ${req.query.name}!</p>`);
}
