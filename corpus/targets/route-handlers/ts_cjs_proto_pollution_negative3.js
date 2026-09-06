// SAFE: Filters out __proto__ and constructor before using Object.assign
var express = require('express');
var app = express();

function sanitize(obj) {
    var safe = {};
    Object.keys(obj).forEach(function(k) {
        if (k !== '__proto__' && k !== 'constructor') {
            safe[k] = obj[k];
        }
    });
    return safe;
}

function merge(target, source) {
    return Object.assign({}, target, sanitize(source));
}

app.post('/merge', function(req, res) {
    var safeDefaults = { theme: 'light', locale: 'en' };
    var merged = merge(safeDefaults, req.body);
    res.json(merged);
});

app.post('/update-config', function(req, res) {
    var config = { mode: 'user' };
    var merged = merge(config, req.body);
    res.json(merged);
});
