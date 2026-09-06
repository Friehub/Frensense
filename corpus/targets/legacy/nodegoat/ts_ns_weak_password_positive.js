// [frensense]
// observation: Password validation uses a weak regex that accepts passwords of only 1-20 characters with no complexity requirements.
// impact: Attackers can brute-force weak passwords easily, leading to account takeover.
// improvement: Enforce a strong password policy requiring at least 8 characters with mixed case, numbers, and special characters.
// cwe: CWE-521
// cvss: 7.5
// owasp: A02:2021

const { environmentalScripts } = require("../../config/config");

function SessionHandler(db) {
    "use strict";

    const validateSignup = (userName, password, verify, errors) => {
        const PASS_RE = /^.{1,20}$/;

        if (!PASS_RE.test(password)) {
            errors.passwordError = "Password must be 8 to 18 characters.";
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