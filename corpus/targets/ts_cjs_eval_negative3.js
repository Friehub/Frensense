// SAFE: Uses JSON.parse with try-catch for safe parsing instead of eval
var express = require('express');
var app = express();

function safeParse(data) {
    try {
        return JSON.parse(data);
    } catch (e) {
        return null;
    }
}

app.post('/parse', function(req, res) {
    var parsed = safeParse(req.body.jsonData);
    if (!parsed) {
        return res.status(400).json({ error: 'Invalid JSON' });
    }
    res.json({ result: parsed });
});

app.get('/config', function(req, res) {
    var cfg = safeParse(req.query.config);
    if (!cfg) {
        return res.status(400).json({ error: 'Invalid config' });
    }
    res.json(cfg);
});
