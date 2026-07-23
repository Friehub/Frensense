var express = require('express');
var app = express();

function merge(target, source) {
    var safeKeys = Object.keys(source).filter(function(k) {
        return k !== "__proto__" && k !== "constructor";
    });
    for (var i = 0; i < safeKeys.length; i++) {
        target[safeKeys[i]] = source[safeKeys[i]];
    }
    return target;
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
