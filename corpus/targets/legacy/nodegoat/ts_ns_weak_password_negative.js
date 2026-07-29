// SAFE: Enforce strong password with minimum length and character class requirements.

const { environmentalScripts } = require("../../config/config");

function SessionHandler(db) {
    "use strict";

    const validateSignup = (userName, password, verify, errors) => {
        const PASS_RE = /^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}$/;

        if (!PASS_RE.test(password)) {
            errors.passwordError = "Password must be at least 8 characters with numbers, lowercase, and uppercase letters.";
            return false;
        }
        if (password !== verify) {
            errors.verifyError = "Password must match";
            return false;
        }
        return true;
    };

    this.handleSignup = (req, res, next) => {
        const { userName, password, verify } = req.body;
        const errors = { userName };
        if (validateSignup(userName, password, verify, errors)) {
            return res.render("dashboard", { environmentalScripts });
        }
        return res.render("signup", { ...errors, environmentalScripts });
    };
}

module.exports = SessionHandler;