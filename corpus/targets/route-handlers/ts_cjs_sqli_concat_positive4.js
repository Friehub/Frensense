// [frensense]
// observation: User input is concatenated directly into SQL queries without parameterization.
// impact: An attacker can execute arbitrary SQL commands by crafting input with SQL metacharacters.
// improvement: Use parameterized queries or prepared statements to separate SQL logic from data.

function UserController(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.getUserById = function(req, res, next) {
        const userId = req.params.id;

        dao.findUser(userId, function(err, result) {
            if (err) return next(err);
            res.render("user", { user: result });
        });
    };

    this.deleteOrder = function(req, res, next) {
        const orderId = req.body.orderId;

        dao.removeOrder(orderId, function(err, result) {
            if (err) return next(err);
            res.render("orders", { success: true });
        });
    };
}

module.exports = UserController;
