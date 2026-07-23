// SAFE: Uses a switch statement with a fixed set of operators instead of eval
var express = require('express');
var app = express();

function calculate(op, a, b) {
    switch (op) {
        case 'add': return a + b;
        case 'subtract': return a - b;
        case 'multiply': return a * b;
        case 'divide': return b !== 0 ? a / b : null;
        default: return null;
    }
}

app.post('/calculate', function(req, res) {
    var a = parseFloat(req.body.a);
    var b = parseFloat(req.body.b);
    var op = req.body.op;
    if (isNaN(a) || isNaN(b)) {
        return res.status(400).json({ error: 'Invalid numbers' });
    }
    var result = calculate(op, a, b);
    if (result === null) {
        return res.status(400).json({ error: 'Invalid operation' });
    }
    res.json({ result: result });
});
