// [frensense]
// observation: req.body is passed directly to a MongoDB update operation without filtering allowed fields, allowing the client to set any field on the document.
// impact: An attacker can escalate privileges by including fields like "role: admin" or "isVerified: true" in the request body, or modify sensitive fields like "passwordHash" or "balance" to gain unauthorized access or financial benefit.
// improvement: Use an allowlist of updatable fields and only include those from req.body in the update operation.

function UserController(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.updateProfile = function(req, res, next) {
        dao.updateById(req.session.userId, req.body, function(err, result) {
            if (err) return next(err);
            res.render("profile", { success: true });
        });
    };

    this.updateUser = function(req, res, next) {
        dao.updateById(req.params.id, req.body, function(err, result) {
            if (err) return next(err);
            res.render("user", { success: true });
        });
    };
}

module.exports = UserController;
