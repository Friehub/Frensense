// SAFE: Wraps background async work in ctx.waitUntil().
// This is NOT a Cloudflare Worker — it's an Express handler that uses await properly.

const express = require("express");

async function handleRequest(req, res) {
    const result = await processAsync(req.body);
    res.json(result);
}

async function processAsync(data) {
    return { processed: true, data };
}

module.exports = { handleRequest };
