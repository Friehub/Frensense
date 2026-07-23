// [frensense]
// observation: User-controlled input is passed to the Function constructor and immediately invoked, allowing arbitrary code execution on the server.
// impact: An attacker can execute arbitrary JavaScript by crafting a malicious string passed to new Function(), leading to complete server compromise, data theft, or remote code execution.
// improvement: Never use the Function constructor with user input. Use whitelisted function maps, sandboxed environments (vm2), or safe expression evaluators.

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/transform', function(req, res) {
    var transformCode = req.query.transform;
    var userData = req.query.data;

    try {
      var transformFn = new Function('data', 'return ' + transformCode);
      var result = transformFn(userData);
      res.json({ result: result });
    } catch (e) {
      res.status(400).json({ error: 'Transform failed' });
    }
  });

  app.post('/api/filter', function(req, res) {
    var filterLogic = req.body.filter;
    var records = req.body.records;

    try {
      var filterFn = new Function('item', filterLogic);
      var filtered = records.filter(filterFn);
      res.json({ results: filtered });
    } catch (e) {
      res.status(400).json({ error: 'Filter failed' });
    }
  });
};
