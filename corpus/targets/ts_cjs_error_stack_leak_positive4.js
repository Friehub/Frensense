// [frensense]
// observation: There is no custom error-handling middleware, so Express's default error handler sends the full stack trace to the client when an error occurs.
// impact: An attacker can trigger an error to leak sensitive information including file paths, module versions, application structure, and internal logic via the stack trace.
// improvement: Add a 4-argument error handler that returns a generic error message and logs the stack internally.

function UserController(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.getUserById = function(req, res, next) {
        const id = parseInt(req.params.id, 10);

        if (id <= 0) {
            throw new Error("Invalid user ID: " + id + " — stack: " + new Error().stack);
        }

        dao.findById(id, function(err, user) {
            if (err) return next(err);
            console.log("stack: " + new Error().stack);
            res.render("user", { user: user });
        });
    };
}

module.exports = UserController;
