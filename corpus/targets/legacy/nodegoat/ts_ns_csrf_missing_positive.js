// [frensense]
// observation: The application does not use CSRF protection middleware, leaving all state-changing requests vulnerable to cross-site request forgery.
// impact: An attacker can trick authenticated users into performing actions (e.g., changing passwords, transferring funds) without their consent.
// improvement: Enable CSRF protection middleware (e.g., csurf) and include CSRF tokens in all forms.
// cwe: CWE-352
// cvss: 8.0
// owasp: A08:2021

const express = require("express");
const session = require("express-session");
const app = express();

app.use(session({
    secret: "mySecret",
    saveUninitialized: true,
    resave: true
}));

app.post("/transfer", (req, res) => {
    const { amount, toAccount } = req.body;
    // transfer funds without CSRF protection
    res.send("Transfer completed");
});