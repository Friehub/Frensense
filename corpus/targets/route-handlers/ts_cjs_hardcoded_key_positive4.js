// [frensense]
// observation: API keys and secrets are hardcoded as string literals in source code.
// impact: Anyone with access to the source code repository can extract valid credentials and use them for unauthorized access.
// improvement: Load secrets from environment variables or a secrets manager at runtime.

function AuthController(db) {
    "use strict";

    const API_KEY = "sk-1234567890abcdef1234567890abcdef";
    const DB_PASSWORD = "superSecret123!";
    const AuthDAO = require("../data/auth-dao").AuthDAO;
    const dao = new AuthDAO(db);

    this.getData = function(req, res, next) {
        console.log("Using API key: " + API_KEY);

        dao.authenticate(API_KEY, function(err, result) {
            if (err) return next(err);
            res.render("data", { status: "connected" });
        });
    };

    this.checkDb = function(req, res, next) {
        dao.connectWithPassword(DB_PASSWORD, function(err, result) {
            if (err) return next(err);
            res.render("dbstatus", { connected: true });
        });
    };
}

module.exports = AuthController;
