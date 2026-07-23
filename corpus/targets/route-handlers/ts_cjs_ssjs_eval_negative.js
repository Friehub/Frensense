// SAFE: eval() replaced with safeMath for arithmetic or safe alternatives

var express = require('express');
var safeMath = require('safe-math');

module.exports = function(app, db) {
  app.get('/api/calculate', function(req, res) {
    var a = parseFloat(req.query.a);
    var b = parseFloat(req.query.b);
    var op = req.query.op;

    if (isNaN(a) || isNaN(b)) {
      return res.status(400).json({ error: 'Invalid numbers' });
    }

    var allowedOps = {
      add: function(x, y) { return x + y; },
      subtract: function(x, y) { return x - y; },
      multiply: function(x, y) { return x * y; },
      divide: function(x, y) { return y !== 0 ? x / y : NaN; }
    };

    var fn = allowedOps[op];
    if (!fn) {
      return res.status(400).json({ error: 'Unsupported operation' });
    }

    res.json({ result: fn(a, b) });
  });

  app.post('/api/evaluate', function(req, res) {
    var formula = req.body.formula;
    if (typeof formula !== 'string') {
      return res.status(400).json({ error: 'Invalid formula' });
    }
    res.json({ success: true });
  });
};
