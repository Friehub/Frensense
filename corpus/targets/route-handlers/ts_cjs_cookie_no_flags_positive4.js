// [frensense]
// observation: Session cookies are set without httpOnly, secure, or sameSite flags.
// impact: An attacker who obtains the cookie via XSS or network sniffing can hijack the user's session.
// improvement: Always set httpOnly, secure, and sameSite flags on session cookies.

function SessionController(db) {
    "use strict";

    const SessionDAO = require("../data/session-dao").SessionDAO;
    const dao = new SessionDAO(db);

    this.login = function(req, res, next) {
        const token = "some-session-value";

        dao.createSession(req.session.userId, token, function(err, result) {
            if (err) return next(err);
            res.cookie("session", token);
            res.render("login", { ok: true });
        });
    };

    this.rememberMe = function(req, res, next) {
        const rememberToken = "remember-" + Date.now();

        dao.saveRememberToken(req.session.userId, rememberToken, function(err, result) {
            if (err) return next(err);
            res.cookie("remember", rememberToken);
            res.render("login", { ok: true });
        });
    };
}

module.exports = SessionController;
