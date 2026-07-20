// SAFE: Uses Object.assign with a null-prototype target to prevent prototype pollution
var express = require('express');
var app = express();

function merge(target, source) {
    var safe = Object.create(null);
    var keys = Object.keys(source);
    for (var i = 0; i < keys.length; i++) {
        safe[keys[i]] = source[keys[i]];
    }
    return Object.assign(target, safe);
}

app.post('/merge', function(req, res) {
    var target = { existing: "data" };
    merge(target, req.body);
    res.json(target);
});

app.post('/update-config', function(req, res) {
    var config = { theme: "default" };
    merge(config, req.body);
    res.json(config);
});
