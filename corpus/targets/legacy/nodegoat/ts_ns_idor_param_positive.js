// [frensense]
// observation: User ID is taken directly from URL parameters (req.params.userId) instead of from the authenticated session.
// impact: An attacker can access another user's data by changing the userId parameter in the URL, leading to Insecure Direct Object Reference (IDOR).
// improvement: Always use the userId from the authenticated session (req.session.userId) rather than from URL parameters.
// cwe: CWE-639
// cvss: 7.5
// owasp: A04:2021

const AllocationsDAO = require("../data/allocations-dao").AllocationsDAO;
const { environmentalScripts } = require("../../config/config");

function AllocationsHandler(db) {
    "use strict";

    const allocationsDAO = new AllocationsDAO(db);

    this.displayAllocations = (req, res, next) => {
        const { userId } = req.params;

        allocationsDAO.getByUserId(userId, (err, allocations) => {
            if (err) return next(err);
            return res.render("allocations", { userId, allocations, environmentalScripts });
        });
    };
}

module.exports = AllocationsHandler;