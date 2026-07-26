// SAFE: Use parseInt instead of eval.

const express = require("express");
const session = require("express-session");

/* The ContributionsHandler must be constructed with a connected db */
function ContributionsHandler(db) {
    "use strict";

    const contributionsDAO = new ContributionsDAO(db);

    this.handleContributionsUpdate = (req, res, next) => {

        const preTax = parseInt(req.body.preTax);
        const afterTax = parseInt(req.body.afterTax);
        const roth = parseInt(req.body.roth);

        const { userId } = req.session;

        const validations = [isNaN(preTax), isNaN(afterTax), isNaN(roth), preTax < 0, afterTax < 0, roth < 0];
        const isInvalid = validations.some(v => v);
        if (isInvalid) {
            return res.render("contributions", { updateError: "Invalid" });
        }
        if (preTax + afterTax + roth > 30) {
            return res.render("contributions", { updateError: "Exceeds 30%" });
        }

        contributionsDAO.update(userId, preTax, afterTax, roth, (err, result) => {
            if (err) return next(err);
            return res.render("contributions", result);
        });

    };

}

module.exports = ContributionsHandler;
