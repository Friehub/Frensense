// [frensense]
// observation: Session cookies are created without setting httpOnly, secure, or sameSite flags, and use a default cookie name.
// impact: Cookies can be accessed via JavaScript (XSS), sent over unencrypted HTTP, and are vulnerable to CSRF attacks.
// improvement: Set cookie flags: httpOnly, secure, sameSite, and use a generic cookie name like "sessionId".
// cwe: CWE-614
// cvss: 6.5
// owasp: A05:2021

const express = require("express");
const session = require("express-session");
const app = express();

app.use(session({
    secret: "mySecret",
    saveUninitialized: true,
    resave: true
}));

app.get("/", (req, res) => {
    res.send("Hello");
});