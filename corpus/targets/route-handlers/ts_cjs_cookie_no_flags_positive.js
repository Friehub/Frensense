// [frensense]
// observation: Session cookies are set without httpOnly, secure, or sameSite flags.
// impact: An attacker who obtains the cookie via XSS or network sniffing can hijack the user's session.
// improvement: Always set httpOnly, secure, and sameSite flags on session cookies.

var express = require('express');
var app = express();

function handler(req, res) {
    var token = "some-session-value";
    res.cookie("session", token); // No security flags
    res.json({ ok: true });
}

app.get('/login', handler);
