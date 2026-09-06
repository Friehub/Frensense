// SAFE: Logs only non-sensitive fields (user ID, action).
// Express handler that avoids logging PII.

const express = require("express");

function logAction(req, res, next) {
    const safeLog = {
        userId: req.session?.userId,
        action: req.path,
        method: req.method,
        timestamp: new Date().toISOString(),
    };
    console.log(JSON.stringify(safeLog));
    next();
}

module.exports = { logAction };
