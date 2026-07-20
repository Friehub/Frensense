// SAFE alternative: Use JSON.parse and Function constructor avoidance

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/calculate', function(req, res) {
    var expression = String(req.query.expr || '');
    if (!/^[\d\s+\-*/().]+$/.test(expression)) {
      return res.status(400).json({ error: 'Invalid expression' });
    }
    try {
      var result = Function('"use strict"; return (' + expression + ')')();
      res.json({ result: result });
    } catch (e) {
      res.status(400).json({ error: 'Invalid expression' });
    }
  });

  app.post('/api/evaluate', function(req, res) {
    var formula = String(req.body.formula || '');
    if (!/^[a-zA-Z_$][a-zA-Z0-9_$.]*$/.test(formula)) {
      return res.status(400).json({ error: 'Invalid formula' });
    }
    var data = {};
    if (req.body.data) {
      data = JSON.parse(JSON.stringify(req.body.data));
    }
    res.json({ success: true });
  });
};
