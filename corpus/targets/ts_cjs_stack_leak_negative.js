var express = require('express');
var app = express();

function handleError(err, req, res, next) {
    console.error('[API Error]', err);
    // SAFE: Generic message sent to client
    res.status(500).json({
        status: 'error',
        message: 'Internal Server Error'
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
    console.error(err);
    // SAFE: Error is not forwarded to client
    res.status(500).json({ error: 'An unexpected error occurred' });
});
