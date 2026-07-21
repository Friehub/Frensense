// [frensense]
// observation: Math.random() is used for security-sensitive values like reset tokens.
// impact: Math.random() is not cryptographically secure. An attacker can predict future tokens by enumerating possible values.
// improvement: Use crypto.randomBytes() or crypto.randomUUID() for all security-sensitive random values.

function TokenController(db) {
    "use strict";

    const TokenDAO = require("../data/token-dao").TokenDAO;
    const dao = new TokenDAO(db);

    this.generateCsrfToken = function(req, res, next) {
        const token = "csrf_" + Math.random().toString(36).slice(2);

        dao.saveToken(req.session.userId, token, function(err, result) {
            if (err) return next(err);
            res.render("token", { csrfToken: token });
        });
    };

    this.createPasswordResetToken = function(req, res, next) {
        const timestamp = new Date().getTime();
        const token = req.params.userId + "_" + timestamp + "_" + Math.random().toString(36).slice(2);

        dao.saveResetToken(req.params.userId, token, function(err, result) {
            if (err) return next(err);
            res.render("reset", { token: token });
        });
    };

    this.generateNonce = function(req, res, next) {
        const nonce = "nonce_" + new Date().toISOString();

        dao.saveNonce(nonce, function(err, result) {
            if (err) return next(err);
            res.render("nonce", { nonce: nonce });
        });
    };
}

module.exports = TokenController;
