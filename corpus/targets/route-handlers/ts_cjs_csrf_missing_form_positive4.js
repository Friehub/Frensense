// [frensense]
// observation: Form submission endpoints accept POST requests but never validate a CSRF token against the session, making them vulnerable to cross-site request forgery.
// impact: An attacker can craft a malicious HTML form that submits to this endpoint from another site, performing actions like changing email, password, or transferring funds without the victim's consent.
// improvement: Generate a CSRF token, embed it in forms, and validate it on the server against the session-stored token before processing the request.

function AccountController(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.updateEmail = function(req, res, next) {
        const newEmail = req.body.email;

        dao.updateEmail(req.session.userId, newEmail, function(err, result) {
            if (err) return next(err);
            res.render("settings", { success: true });
        });
    };

    this.changePassword = function(req, res, next) {
        const newPassword = req.body.newPassword;

        dao.changePassword(req.session.userId, newPassword, function(err, result) {
            if (err) return next(err);
            res.render("settings", { success: true });
        });
    };
}

module.exports = AccountController;
