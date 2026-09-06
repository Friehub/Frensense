// [frensense]
// observation: The application creates an HTTP server instead of HTTPS, transmitting all data in plaintext.
// impact: Sensitive data including passwords, session cookies, and personal information can be intercepted by attackers on the network.
// improvement: Use HTTPS (https.createServer) with valid TLS certificates to encrypt all traffic.
// cwe: CWE-319
// cvss: 7.4
// owasp: A05:2021

const express = require("express");
const http = require("http");
const app = express();

app.get("/", (req, res) => {
    res.send("Hello");
});

http.createServer(app).listen(3000);