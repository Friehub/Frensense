// SAFE alternative: Use vm module in sandboxed context with timeout

var express = require('express');
var vm = require('vm');

module.exports = function(app, db) {
  app.get('/api/transform', function(req, res) {
    var transformCode = String(req.query.transform || '');
    var userData = req.query.data;

    var allowedPattern = /^[a-zA-Z_$][a-zA-Z0-9_$]*\s*\(?\s*\)?\s*$/;
    if (!allowedPattern.test(transformCode)) {
      return res.status(400).json({ error: 'Invalid transform' });
    }

    var sandbox = {
      data: userData,
      result: null
    };

    try {
      vm.runInNewContext('result = ' + transformCode + '(data)', sandbox, { timeout: 100 });
      res.json({ result: sandbox.result });
    } catch (e) {
      res.status(400).json({ error: 'Transform failed' });
    }
  });

  app.post('/api/filter', function(req, res) {
    var filterLogic = String(req.body.filter || '');
    var records = req.body.records;

    if (!Array.isArray(records)) {
      return res.status(400).json({ error: 'Records must be an array' });
    }

    if (!/^[a-zA-Z_$][a-zA-Z0-9_$]*$/.test(filterLogic)) {
      return res.status(400).json({ error: 'Invalid filter' });
    }

    var sandbox = {
      records: records,
      results: []
    };

    try {
      vm.runInNewContext('results = records.filter(function(item) { return ' + filterLogic + '(item); })', sandbox, { timeout: 100 });
      res.json({ results: sandbox.results });
    } catch (e) {
      res.status(400).json({ error: 'Filter failed' });
    }
  });
};
