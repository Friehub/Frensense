// [frensense]
// observation: The session is not regenerated after login, leaving the old session ID active with new privileges.
// impact: An attacker who obtained the session ID before login can continue using it after the user authenticates, leading to session hijacking.
// improvement: Call req.session.regenerate() before assigning userId to the session after successful login.
// cwe: CWE-384
// cvss: 7.5
// owasp: A02:2021

const UserDAO = require("../data/user-dao").UserDAO;

function SessionHandler(db) {
    "use strict";

    const userDAO = new UserDAO(db);

    this.handleLoginRequest = (req, res, next) => {
        const { userName, password } = req.body;
        userDAO.validateLogin(userName, password, (err, user) => {
            if (err) return next(err);

            req.session.userId = user._id;
            return res.redirect(user.isAdmin ? "/benefits" : "/dashboard");
        });
    };
}

module.exports = SessionHandler;