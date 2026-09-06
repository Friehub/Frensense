// [frensense]
// observation: The default X-Powered-By response header is not disabled, revealing server technology information.
// impact: An attacker can identify the server framework and version, enabling targeted attacks against known vulnerabilities.
// improvement: Disable the X-Powered-By header using app.disable("x-powered-by").
// cwe: CWE-200
// cvss: 2.6
// owasp: A05:2021

const express = require("express");
const app = express();

app.get("/", (req, res) => {
    res.send("Hello");
});