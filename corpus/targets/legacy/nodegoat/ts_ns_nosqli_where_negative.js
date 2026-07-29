// SAFE: Use numeric operators instead of $where, parse threshold as integer.

const { environmentalScripts } = require("../../config/config");

function AllocationsHandler(db) {
    "use strict";

    const allocationsDAO = new AllocationsDAO(db);

    this.displayAllocations = (req, res, next) => {
        const { userId } = req.session;
        const { threshold } = req.query;

        const parsedThreshold = parseInt(threshold, 10);
        const searchCriteria = () => {
            if (!isNaN(parsedThreshold) && parsedThreshold >= 0 && parsedThreshold <= 99) {
                return { userId: parseInt(userId), stocks: { $gt: parsedThreshold } };
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