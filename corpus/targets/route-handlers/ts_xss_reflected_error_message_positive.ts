// [frensense]
// observation: A user-controlled error message is rendered in the response without sanitization, allowing script injection via error text.
// impact: An attacker can craft a request that triggers an error with a malicious payload, resulting in XSS when the error page renders the message.
// improvement: Never render user input directly in error responses; use generic error messages and encode any dynamic content.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function errorHandler(err: any, req: express.Request, res: express.Response) {
    const message = req.query.error as string || err.message;
    res.send(`<div class="error">Error: ${message}</div>`);
}

export function notFoundHandler(req: express.Request, res: express.Response) {
    const path = req.originalUrl;
    res.status(404).send(`<h1>404 - ${path} not found</h1>`);
}
