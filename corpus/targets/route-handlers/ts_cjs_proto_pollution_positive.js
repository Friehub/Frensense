// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys through an intermediate variable.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes.
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target.

var _ = require('lodash');
var express = require('express');
var app = express();

function handleMerge(req, res) {
    var target = { existing: "data" };
    _.merge(target, req.body);
    res.json(target);
}

function handleUpdateConfig(req, res) {
    var config = { theme: "default" };
    _.merge(config, req.body);
    res.json(config);
}

app.post('/merge', handleMerge);
app.post('/update-config', handleUpdateConfig);
