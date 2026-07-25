// [frensense]
// observation: User-controlled input is directly interpolated into HTML response without escaping with renamed variables.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS).
// improvement: Encode all user input before embedding in HTML
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    const searchQuery = req.query.q as string;
    res.send(`<html><body><h1>Search results for: ${searchQuery}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const userName = req.query.name as string;
    res.send(`<p>Welcome, ${userName}!</p>`);
}
