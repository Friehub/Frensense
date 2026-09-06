// [frensense]
// observation: The helmet security middleware is not used, leaving multiple HTTP headers insecure (no CSP, no HSTS, no frameguard, no XSS filter).
// impact: The application is vulnerable to clickjacking, MIME-sniffing, XSS, and other browser-level attacks.
// improvement: Use helmet middleware to set secure HTTP headers.
// cwe: CWE-1021
// cvss: 5.3
// owasp: A05:2021

const express = require("express");
const app = express();

app.get("/", (req, res) => {
    res.send("Hello");
});