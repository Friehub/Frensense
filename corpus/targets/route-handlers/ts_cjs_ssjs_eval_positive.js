// [frensense]
// observation: User input from query parameters is passed directly to eval(), allowing arbitrary server-side JavaScript execution.
// impact: An attacker can execute arbitrary code on the server by passing JavaScript to the eval parameter, leading to full server compromise, data exfiltration, or remote code execution.
// improvement: Never use eval() with user input. Use safe alternatives like JSON.parse() for JSON, or whitelist-based function lookups for dynamic behavior.
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/calculate', function(req, res) {
    var expression = req.query.expr;
    try {
      var result = eval(expression);
      res.json({ result: result });
    } catch (e) {
      res.status(400).json({ error: 'Invalid expression' });
    }
  });

  app.post('/api/evaluate', function(req, res) {
    var formula = req.body.formula;
    var data = req.body.data;
    try {
      eval('data.' + formula);
      res.json({ success: true });
    } catch (e) {
      res.status(400).json({ error: 'Evaluation failed' });
    }
  });
};
