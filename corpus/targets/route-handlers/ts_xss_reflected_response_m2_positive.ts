// [frensense]
// observation: User-controlled input from a query parameter flows through an intermediate variable into the HTML response without escaping.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: HTML-escape all user input before embedding it in the response.

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    const query = req.query.q as string;
    res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = req.query.name as string;
    res.send(`<p>Welcome, ${name}!</p>`);
}
