// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query via string interpolation.
// impact: An attacker can inject arbitrary JavaScript code into the NoSQL query, potentially extracting sensitive data or performing denial of service.
// improvement: Use query operators that do not execute JavaScript (e.g., $gt, $lt) and validate input as numeric before use.
// cwe: CWE-943
// cvss: 8.1
// owasp: A01:2021

const { environmentalScripts } = require("../../config/config");

function AllocationsHandler(db) {
    "use strict";

    const allocationsDAO = new AllocationsDAO(db);

    this.displayAllocations = (req, res, next) => {
        const { userId } = req.params;
        const { threshold } = req.query;

        const searchCriteria = () => {
            if (threshold) {
                return {
                    $where: `this.userId == ${userId} && this.stocks > '${threshold}'`
                };
            }
            return { userId: parseInt(userId) };
        };

        allocationsDAO.getByUserId(userId, (err, allocations) => {
            if (err) return next(err);
            return res.render("allocations", { userId, allocations, environmentalScripts });
        });
    };
}

module.exports = AllocationsHandler;