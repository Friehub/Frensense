// [frensense]
// observation: MongoDB $where operator directly interpolates a user-controlled condition from the request body, enabling server-side JavaScript injection.
// impact: An attacker can craft a malicious $where string that executes arbitrary JavaScript in the MongoDB context, potentially extracting data from other collections.
// improvement: Remove $where from queries entirely; use standard MongoDB operators like $eq, $gt with validated input.

function NoSqlWhereHandler(db) {
    "use strict";

    this.search = function(req, res, next) {
        const searchTerm = req.query.q;
        db.collection("users").find({
            $where: "this.username.indexOf(\"" + searchTerm + "\") !== -1"
        }).toArray(function(err, users) {
            if (err) return next(err);
            res.json(users);
        });
    };

    this.filter = function(req, res, next) {
        const minAmount = req.query.min;
        db.collection("orders").find({
            $where: "this.total >= " + minAmount
        }).toArray(function(err, orders) {
            if (err) return next(err);
            res.json(orders);
        });
    };
}

module.exports = NoSqlWhereHandler;
