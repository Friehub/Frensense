// [frensense]
// observation: User-controlled input is concatenated into the HTML response string without escaping.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response via string concatenation, enabling XSS.
// improvement: HTML-escape all user input before concatenation into response strings.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
    res.send("<html><body><h1>Search results for: " + req.query.q + "</h1></body></html>");
}

export function greetingHandler(req: express.Request, res: express.Response) {
    res.send("<p>Welcome, " + req.query.name + "!</p>");
}
