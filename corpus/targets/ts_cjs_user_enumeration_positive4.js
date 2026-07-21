// [frensense]
// observation: Login endpoint returns distinct error messages for nonexistent username versus wrong password, leaking which usernames are registered.
// impact: An attacker can systematically enumerate valid usernames via the error messages, then launch targeted password brute-force attacks.
// improvement: Return a single generic error message for all authentication failures regardless of the specific cause.

function UserEnumHandler(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.login = function(req, res, next) {
        const username = req.body.username;
        const password = req.body.password;

        dao.findByUsername(username, function(err, user) {
            if (err) return next(err);
            if (!user) {
                return res.status(401).json({ error: "Username not found" });
            }
            if (user.password !== password) {
                return res.status(401).json({ error: "Incorrect password" });
            }
            req.session.userId = user._id;
            res.json({ success: true });
        });
    };
}

module.exports = UserEnumHandler;
