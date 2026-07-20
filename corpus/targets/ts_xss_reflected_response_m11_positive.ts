// [frensense]
// observation: User-controlled input is directly interpolated into the HTML response without escaping inside a conditional block on the tainted branch.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS).
// improvement: Encode all user input before embedding in HTML

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    if (req.query.q) {
        res.send(`<html><body><h1>Search results for: ${req.query.q}</h1></body></html>`);
    } else { res.send(`<html><body><h1>No search query</h1></body></html>`); }
}

export function greetingHandler(req: express.Request, res: express.Response) {
    if (req.query.name) {
        res.send(`<p>Welcome, ${req.query.name}!</p>`);
    }
}
