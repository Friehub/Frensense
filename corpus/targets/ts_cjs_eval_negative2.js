// SAFE: Uses mathjs expression parser instead of eval
var math = require('mathjs');
var express = require('express');
var app = express();

app.post('/calculate', function(req, res) {
    var result = math.evaluate(req.body.expression);
    res.json({ result: result });
});

app.get('/run', function(req, res) {
    var result = math.evaluate(req.query.code);
    res.json({ result: result });
});
