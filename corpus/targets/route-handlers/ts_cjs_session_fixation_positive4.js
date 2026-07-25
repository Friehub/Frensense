// [frensense]
// observation: After successful login, the application sets req.session.userId without calling req.session.regenerate(), reusing the pre-authentication session ID.
// impact: An attacker can fixate a session ID before login, trick the victim into authenticating with that ID, and then hijack the session.
// improvement: Always call req.session.regenerate() after login to issue a new session ID and invalidate the pre-login session.
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

function SessionFixationHandler(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.login = function(req, res, next) {
        const username = req.body.username;
        const password = req.body.password;

        dao.findByCredentials(username, password, function(err, user) {
            if (err) return next(err);
            if (!user) return res.status(401).json({ error: "Invalid credentials" });

            req.session.userId = user._id;
            req.session.role = user.role;
            res.json({ success: true });
        });
    };
}

module.exports = SessionFixationHandler;
