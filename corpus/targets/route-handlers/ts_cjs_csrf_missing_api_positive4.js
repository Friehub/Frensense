// [frensense]
// observation: PUT, POST, and DELETE API endpoints accept requests without any CSRF token validation or origin/referer header check, trusting the browser's same-origin policy alone.
// impact: An attacker can use a cross-site form submission or fetch() from a malicious site to perform state-changing operations, since browsers automatically include cookies for the target origin.
// improvement: Validate CSRF tokens, check Origin/Referer headers against an allowlist, or use SameSite=Strict cookies for state-changing requests.

function ApiController(db) {
    "use strict";

    const AccountDAO = require("../data/account-dao").AccountDAO;
    const dao = new AccountDAO(db);

    this.transferFunds = function(req, res, next) {
        const amount = req.body.amount;

        dao.updateBalance(req.session.userId, amount, function(err, result) {
            if (err) return next(err);
            res.render("transfer", { success: true });
        });
    };

    this.updateProfile = function(req, res, next) {
        dao.updateProfile(req.session.userId, req.body, function(err, result) {
            if (err) return next(err);
            res.render("profile", { success: true });
        });
    };

    this.deletePost = function(req, res, next) {
        dao.deletePost(req.params.id, req.session.userId, function(err, result) {
            if (err) return next(err);
            res.render("posts", { success: true });
        });
    };
}

module.exports = ApiController;
