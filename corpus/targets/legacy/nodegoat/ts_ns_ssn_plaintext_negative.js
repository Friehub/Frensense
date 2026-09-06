// SAFE: Encrypt sensitive fields before storing.

const crypto = require("crypto");

function encrypt(text, key, algo) {
    const iv = crypto.randomBytes(16);
    const cipher = crypto.createCipheriv(algo, key, iv);
    return cipher.update(text, "utf8", "hex") + cipher.final("hex");
}

function ProfileDAO(db) {
    "use strict";

    const users = db.collection("users");

    this.updateUser = (userId, firstName, lastName, ssn, dob, address, bankAcc, bankRouting, callback) => {
        const user = {};
        if (firstName) user.firstName = firstName;
        if (lastName) user.lastName = lastName;
        if (address) user.address = address;
        if (ssn) user.ssn = encrypt(ssn, "encryptionKey", "aes-256-cbc");
        if (dob) user.dob = encrypt(dob, "encryptionKey", "aes-256-cbc");

        users.update({ _id: parseInt(userId) }, { $set: user }, err => {
            if (!err) return callback(null, user);
            return callback(err, null);
        });
    };
}

module.exports = { ProfileDAO };