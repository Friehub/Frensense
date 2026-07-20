// SAFE: Uses lodash pick to only select allowed fields before merging
var _ = require('lodash');
var express = require('express');
var app = express();

var allowedFields = ['theme', 'locale', 'notifications'];

function mergeWithAllowlist(target, source) {
    var picked = _.pick(source, allowedFields);
    return Object.assign({}, target, picked);
}

app.post('/merge', function(req, res) {
    var safeDefaults = { theme: 'light', locale: 'en' };
    var merged = mergeWithAllowlist(safeDefaults, req.body);
    res.json(merged);
});

app.post('/update-config', function(req, res) {
    var config = { mode: 'user' };
    var merged = mergeWithAllowlist(config, req.body);
    res.json(merged);
});
