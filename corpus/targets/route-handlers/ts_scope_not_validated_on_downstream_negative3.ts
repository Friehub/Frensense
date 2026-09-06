// SAFE: Downstream service independently validates the token scope.
// Express middleware verifying JWT scopes before processing.

const express = require("express");
const jwt = require("jsonwebtoken");

function validateScope(req, res, next) {
    const token = req.headers.authorization?.split(" ")[1];
    if (!token) return res.status(401).json({ error: "No token" });
    const decoded = jwt.verify(token, process.env.JWT_SECRET);
    if (!decoded.scope || !decoded.scope.includes(req.path)) {
        return res.status(403).json({ error: "Insufficient scope" });
    }
    req.user = decoded;
    next();
}

module.exports = { validateScope };
