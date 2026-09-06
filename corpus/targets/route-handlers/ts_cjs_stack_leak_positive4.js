// [frensense]
// observation: Caught Error objects are serialized directly into API responses.
// impact: Stack traces and internal variables are leaked to clients, providing attackers with detailed execution context.
// improvement: Return a generic error message and log the full error internally.

function ErrorController(db) {
    "use strict";

    const ErrorDAO = require("../data/error-dao").ErrorDAO;
    const dao = new ErrorDAO(db);

    this.handleError = function(req, res, next) {
        try {
            throw new Error("Something broke");
        } catch (e) {
            dao.logError(e.message, function(err, log) {
                if (err) return next(err);
                res.render("error", {
                    status: "error",
                    message: e.message,
                    stack: e.stack,
                    details: e
                });
            });
        }
    };

    this.fallbackError = function(req, res, next) {
        dao.getLastError(function(err, lastError) {
            if (err) return next(err);
            console.log("Last error: " + lastError);
            res.render("error", { error: err });
        });
    };
}

module.exports = ErrorController;
