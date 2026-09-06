// SAFE: Uses crypto.randomBytes instead of Math.random for tokens.
// NOT an LLM/NodeGoat-style token generator — Express route handler with secure random.

const crypto = require("crypto");
const express = require("express");

function generateToken(length) {
    return crypto.randomBytes(length).toString("hex");
}

module.exports = { generateToken };
