// SAFE: Uses crypto.randomUUID instead of Date.now() for tokens.
// Express-style handler generating secure tokens.

const crypto = require("crypto");
const express = require("express");

function generateResetToken() {
    return crypto.randomUUID();
}

module.exports = { generateResetToken };
