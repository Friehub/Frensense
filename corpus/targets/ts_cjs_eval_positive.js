// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution through an intermediate variable.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input.
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs.

var express = require('express');
var app = express();

function handleCalculate(req, res) {
    var val = req.body.expression;
    var result = eval(val);
    res.json({ result: result });
}

function handleRun(req, res) {
    var val = req.query.code;
    var result = eval(val);
    res.json({ result: result });
}

app.post('/calculate', handleCalculate);
app.get('/run', handleRun);
