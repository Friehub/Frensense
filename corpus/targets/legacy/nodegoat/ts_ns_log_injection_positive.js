// [frensense]
// observation: User input from the login form is logged directly via console.log without encoding for CRLF injection.
// impact: An attacker can inject newlines and forge log entries, potentially hiding malicious activity or tricking log reviewers.
// improvement: Encode or sanitize user input before logging to prevent CRLF injection.
// cwe: CWE-117
// cvss: 5.3
// owasp: A01:2021

const UserDAO = require("../data/user-dao").UserDAO;
const { environmentalScripts } = require("../../config/config");

function SessionHandler(db) {
    "use strict";

    const userDAO = new UserDAO(db);

    this.handleLoginRequest = (req, res, next) => {
        const { userName, password } = req.body;
        userDAO.validateLogin(userName, password, (err, user) => {
            if (err) {
                if (err.noSuchUser) {
                    console.log("Error: attempt to login with invalid user: ", userName);
                    return res.render("login", {
                        userName: userName,
                        loginError: "Invalid username",
                        environmentalScripts
                    });
                }
                return next(err);
            }
            req.session.userId = user._id;
            return res.redirect(user.isAdmin ? "/benefits" : "/dashboard");
        });
    };
}

module.exports = SessionHandler;