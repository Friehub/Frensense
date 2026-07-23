var express = require('express');
var app = express();

function processExpression(expr) {
    var sanitized = expr.replace(/[^0-9+\-*\/() ]/g, "");
    return new Function('"use strict"; return (' + sanitized + ')')();
}

app.post('/calculate', function(req, res) {
    var result = processExpression(req.body.expression);
    res.json({ result: result });
});

app.get('/run', function(req, res) {
    var result = processExpression(req.query.code);
    res.json({ result: result });
});
