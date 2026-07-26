// SAFE: BFF validates origin IP and user-agent on each request.
// This negative is an Express middleware that checks session binding.

const express = require("express");

function validateSession(req, res, next) {
    const session = req.session;
    if (!session) return res.status(401).json({ error: "No session" });
    if (session.ip !== req.ip || session.ua !== req.headers["user-agent"]) {
        return res.status(401).json({ error: "Session mismatch" });
    }
    next();
}

module.exports = { validateSession };
