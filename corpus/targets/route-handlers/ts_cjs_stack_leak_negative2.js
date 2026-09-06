// SAFE: Strips stack trace and error details before sending response
var express = require('express');
var app = express();

function handleError(err, req, res, next) {
    var sanitized = { status: "error", message: "Internal Server Error" };
    res.status(500).json(sanitized);
}

app.get('/fail', function(req, res) {
    try {
        throw new Error("Something broke");
    } catch (e) {
        handleError(e, req, res);
    }
});

app.use(function(err, req, res, next) {
    console.error(err, { path: req.path });
    res.status(500).json({ error: "Internal Server Error" });
});
