// [frensense]
// observation: Authentication handler decodes the JWT token using jwt.decode() instead of jwt.verify(), accepting any token without signature validation.
// impact: An attacker can forge arbitrary JWTs with any payload (e.g., { role: "admin" }) to impersonate users or escalate privileges.
// improvement: Replace jwt.decode() with jwt.verify(token, secret) to cryptographically validate the token's signature.
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

function JwtHandler(db) {
    "use strict";

    const jwt = require("jsonwebtoken");
    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.authenticate = function(req, res, next) {
        const authHeader = req.headers.authorization;
        if (!authHeader) return res.status(401).json({ error: "No token" });
        const token = authHeader.split(" ")[1];
        const decoded = jwt.decode(token);
        req.user = decoded;
        next();
    };

    this.dashboard = function(req, res, next) {
        const token = req.cookies.token;
        const payload = jwt.decode(token);
        dao.findById(payload.sub, function(err, user) {
            if (err) return next(err);
            res.render("dashboard", { user: user });
        });
    };
}

module.exports = JwtHandler;
