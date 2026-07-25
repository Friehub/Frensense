// [frensense]
// observation: User-controlled input is directly interpolated into HTML response without escaping inside a try-catch block.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS), with errors silently caught.
// improvement: Encode all user input before embedding in HTML
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    try { res.send(`<html><body><h1>Search results for: ${req.query.q}</h1></body></html>`); } catch (err) { console.error(err); }
}

export function greetingHandler(req: express.Request, res: express.Response) {
    try { res.send(`<p>Welcome, ${req.query.name}!</p>`); } catch {}
}
