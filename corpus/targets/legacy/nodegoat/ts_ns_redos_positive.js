// [frensense]
// observation: A regex pattern uses a nested quantifier ([0-9]+)+ that causes catastrophic backtracking on non-matching input.
// impact: An attacker can craft input that causes the regex evaluation to consume exponential CPU time, leading to denial of service.
// improvement: Remove the nested quantifier; use a non-possessive quantifier or re-anchor the regex to avoid backtracking.
// cwe: CWE-1333
// cvss: 7.5
// owasp: A01:2021

function ProfileHandler(db) {
    "use strict";

    this.handleProfileUpdate = (req, res, next) => {
        const { bankRouting } = req.body;
        const regexPattern = /([0-9]+)+\#/;
        const isValid = regexPattern.test(bankRouting);

        if (isValid !== true) {
            return res.render("profile", { updateError: "Invalid routing number" });
        }

        return res.render("profile", { updateSuccess: true });
    };
}

module.exports = ProfileHandler;