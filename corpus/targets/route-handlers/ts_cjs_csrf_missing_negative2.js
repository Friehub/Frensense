// SAFE: Uses doubleCsrf for automatic CSRF protection
var { doubleCsrf } = require('csrf-csrf');
var express = require('express');
var app = express();

var csrf = doubleCsrf({
    getSecret: function() { return process.env.CSRF_SECRET; },
    cookieName: 'csrf-token',
    cookieOptions: { httpOnly: true, sameSite: 'strict', secure: true }
});

app.use(csrf.doubleCsrfProtection);

app.get('/api/csrf-token', function(req, res) {
    res.json({ token: csrf.generateToken(req, res) });
});
