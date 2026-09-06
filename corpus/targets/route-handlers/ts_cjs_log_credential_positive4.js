// [frensense]
// observation: The application logs sensitive credentials such as passwords, tokens, or API keys.
// impact: Anyone with access to logs (developers, SIEM systems, log management services) can extract valid credentials.
// improvement: Redact sensitive fields before logging, or use structured logging that filters known sensitive keys.

function AuthController(db) {
    "use strict";

    const AuthDAO = require("../data/auth-dao").AuthDAO;
    const dao = new AuthDAO(db);

    this.handleLogin = function(req, res, next) {
        const username = req.body.username;
        const password = req.body.password;

        console.log("Login attempt for " + username + " with password: " + password);

        dao.findByCredentials(username, password, function(err, user) {
            if (err) return next(err);
            res.render("login", { success: true });
        });
    };

    this.processAuth = function(req, res, next) {
        const authHeader = req.headers.authorization;
        const token = req.cookies.session;

        console.log("Auth header: " + authHeader);
        console.log("Session token: " + token);

        dao.validateToken(token, function(err, result) {
            if (err) return next(err);
            res.render("auth", { ok: true });
        });
    };
}

module.exports = AuthController;
