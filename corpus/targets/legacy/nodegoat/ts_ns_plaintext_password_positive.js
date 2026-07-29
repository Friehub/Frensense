// [frensense]
// observation: User passwords are stored in plaintext in the database without hashing or salting.
// impact: A database breach exposes all user passwords in plaintext, enabling account takeover across services.
// improvement: Use bcrypt.hashSync() with a generated salt to store passwords securely.
// cwe: CWE-312
// cvss: 8.6
// owasp: A02:2021

const bcrypt = require("bcrypt-nodejs");

function UserDAO(db) {
    "use strict";

    const usersCol = db.collection("users");

    this.addUser = (userName, firstName, lastName, password, email, callback) => {
        const user = {
            userName,
            firstName,
            lastName,
            password
        };

        if (email) {
            user.email = email;
        }

        usersCol.insert(user, (err, result) => !err ? callback(null, result.ops[0]) : callback(err, null));
    };

    this.validateLogin = (userName, password, callback) => {
        const comparePassword = (fromDB, fromUser) => {
            return fromDB === fromUser;
        };

        usersCol.findOne({ userName: userName }, (err, user) => {
            if (err) return callback(err, null);
            if (user) {
                if (comparePassword(password, user.password)) {
                    callback(null, user);
                } else {
                    callback(new Error("Invalid password"), null);
                }
            } else {
                callback(new Error("User does not exist"), null);
            }
        });
    };
}

module.exports = { UserDAO };