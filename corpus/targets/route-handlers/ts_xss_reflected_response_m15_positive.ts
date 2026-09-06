// [frensense]
// observation: User-controlled input is directly interpolated into HTML response without escaping via a promise .then() chain.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS).
// improvement: Encode all user input before embedding in HTML
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    Promise.resolve(req.query.q as string).then(query => {
        res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
    });
}

export function greetingHandler(req: express.Request, res: express.Response) {
    new Promise<string>(resolve => resolve(req.query.name as string)).then(name => {
        res.send(`<p>Welcome, ${name}!</p>`);
    });
}
