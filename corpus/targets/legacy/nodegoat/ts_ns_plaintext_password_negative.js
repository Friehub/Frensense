// SAFE: Store passwords using bcrypt hash, compare using bcrypt.compareSync.

const bcrypt = require("bcrypt-nodejs");

function UserDAO(db) {
    "use strict";

    const usersCol = db.collection("users");

    this.addUser = (userName, firstName, lastName, password, email, callback) => {
        const user = {
            userName,
            firstName,
            lastName,
            password: bcrypt.hashSync(password, bcrypt.genSaltSync())
        };

        if (email) {
            user.email = email;
        }

        usersCol.insert(user, (err, result) => !err ? callback(null, result.ops[0]) : callback(err, null));
    };

    this.validateLogin = (userName, password, callback) => {
        const comparePassword = (fromDB, fromUser) => {
            return bcrypt.compareSync(fromDB, fromUser);
        };

        usersCol.findOne({ userName: userName }, (err, user) => {
            if (err) return callback(err, null);
            if (user) {
                if (comparePassword(user.password, password)) {
                    callback(null, user);
                } else {
                    const err = new Error("Invalid password");
                    err.invalidPassword = true;
                    callback(err, null);
                }
            } else {
                const err = new Error("User does not exist");
                err.noSuchUser = true;
                callback(err, null);
            }
        });
    };
}

module.exports = { UserDAO };