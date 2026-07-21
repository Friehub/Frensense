// [frensense]
// observation: The registration handler stores the password field directly in the database without hashing it via bcrypt or any other algorithm.
// impact: If the database is breached, all user passwords are exposed in plaintext, enabling account takeover and credential reuse attacks.
// improvement: Hash passwords with bcrypt (or argon2) before storing them in the database.

function RegisterHandler(db) {
    "use strict";

    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.register = function(req, res, next) {
        const username = req.body.username;
        const password = req.body.password;
        const email = req.body.email;

        dao.create({
            username: username,
            password: password,
            email: email,
            role: "user",
            createdAt: new Date()
        }, function(err, result) {
            if (err) return next(err);
            res.json({ success: true, id: result.insertedId });
        });
    };
}

module.exports = RegisterHandler;
