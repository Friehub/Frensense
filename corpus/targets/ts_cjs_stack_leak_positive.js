// [frensense]
// observation: Caught Error objects are serialized directly into API responses.
// impact: Stack traces and internal variables are leaked to clients, providing attackers with detailed execution context.
// improvement: Return a generic error message and log the full error internally.

var express = require('express');
var app = express();

function handleError(err, req, res, next) {
    // VULNERABLE: stack trace leak
    res.status(500).json({
        status: 'error',
        message: err.message,
        stack: err.stack,
        details: err
    });
}

app.get('/fail', function(req, res) {
    try {
        throw new Error("Something broke");
    } catch (e) {
        handleError(e, req, res);
    }
});

app.use(function(err, req, res, next) {
    // VULNERABLE: sending raw error to client
    res.status(500).json({ error: err });
});
