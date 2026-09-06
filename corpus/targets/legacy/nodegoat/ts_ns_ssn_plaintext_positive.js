// [frensense]
// observation: Sensitive personal data (SSN, DOB) is stored in the database without encryption.
// impact: A database breach exposes sensitive personal information in plaintext, leading to identity theft.
// improvement: Encrypt sensitive fields like ssn and dob before storing them using a strong encryption algorithm.
// cwe: CWE-312
// cvss: 7.5
// owasp: A06:2021

function ProfileDAO(db) {
    "use strict";

    const users = db.collection("users");

    this.updateUser = (userId, firstName, lastName, ssn, dob, address, bankAcc, bankRouting, callback) => {
        const user = {};
        if (firstName) user.firstName = firstName;
        if (lastName) user.lastName = lastName;
        if (address) user.address = address;
        if (ssn) user.ssn = ssn;
        if (dob) user.dob = dob;

        users.update({ _id: parseInt(userId) }, { $set: user }, err => {
            if (!err) return callback(null, user);
            return callback(err, null);
        });
    };
}

module.exports = { ProfileDAO };