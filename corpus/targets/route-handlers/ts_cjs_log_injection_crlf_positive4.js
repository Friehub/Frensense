// [frensense]
// observation: User-controlled input is concatenated directly into console.log() calls without stripping CRLF characters, allowing log injection.
// impact: An attacker can inject fake log entries via CRLF in the username field, corrupting log analysis and potentially framing legitimate users.
// improvement: Remove or escape CRLF characters from user input before logging, or use structured parameterized logging.
// cwe: CWE-117
// cvss: 5.3
// owasp: A09:2021
// severity: Medium

function LogHandler(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.login = function(req, res, next) {
        const username = req.body.username;
        const password = req.body.password;

        dao.findByUsername(username, function(err, user) {
            if (err) return next(err);
            if (!user || user.password !== password) {
                console.log("Login failed for user: " + username);
                return res.status(401).json({ error: "Invalid credentials" });
            }
            console.log("Login successful: " + username);
            req.session.userId = user._id;
            res.json({ success: true });
        });
    };
}

module.exports = LogHandler;
