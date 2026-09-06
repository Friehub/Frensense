// [frensense]
// observation: The application does not set the X-Content-Type-Options: nosniff header, allowing browsers to MIME-sniff responses.
// impact: An attacker can upload a malicious file that the browser interprets as a different MIME type, leading to XSS or other attacks.
// improvement: Use helmet.noSniff() or set the X-Content-Type-Options: nosniff header manually.
// cwe: CWE-200
// cvss: 4.3
// owasp: A05:2021

const express = require("express");
const app = express();

app.get("/", (req, res) => {
    res.send("Hello");
});