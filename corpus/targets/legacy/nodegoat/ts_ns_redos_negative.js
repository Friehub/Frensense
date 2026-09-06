// SAFE: Remove nested quantifier to prevent catastrophic backtracking.

function ProfileHandler(db) {
    "use strict";

    this.handleProfileUpdate = (req, res, next) => {
        const { bankRouting } = req.body;
        const regexPattern = /([0-9]+)\#/;
        const isValid = regexPattern.test(bankRouting);

        if (isValid !== true) {
            return res.render("profile", { updateError: "Invalid routing number" });
        }

        return res.render("profile", { updateSuccess: true });
    };
}

module.exports = ProfileHandler;