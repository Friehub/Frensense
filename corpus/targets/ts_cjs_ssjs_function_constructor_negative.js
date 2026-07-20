// SAFE: Predefined transform functions used instead of Function constructor

var express = require('express');

var transforms = {
  uppercase: function(data) { return String(data).toUpperCase(); },
  lowercase: function(data) { return String(data).toLowerCase(); },
  trim: function(data) { return String(data).trim(); },
  reverse: function(data) { return String(data).split('').reverse().join(''); },
  length: function(data) { return String(data).length; },
  json: function(data) { return JSON.parse(String(data)); }
};

module.exports = function(app, db) {
  app.get('/api/transform', function(req, res) {
    var transformName = req.query.transform;
    var userData = req.query.data;

    var transformFn = transforms[transformName];
    if (!transformFn) {
      return res.status(400).json({ error: 'Unknown transform: ' + transformName });
    }

    try {
      var result = transformFn(userData);
      res.json({ result: result });
    } catch (e) {
      res.status(400).json({ error: 'Transform failed' });
    }
  });

  app.post('/api/filter', function(req, res) {
    var filterType = req.body.filter;
    var records = req.body.records;

    if (!Array.isArray(records)) {
      return res.status(400).json({ error: 'Records must be an array' });
    }

    var filters = {
      nonEmpty: function(item) { return item !== null && item !== undefined && item !== ''; },
      numbers: function(item) { return typeof item === 'number'; },
      strings: function(item) { return typeof item === 'string'; }
    };

    var filterFn = filters[filterType];
    if (!filterFn) {
      return res.status(400).json({ error: 'Unknown filter' });
    }

    res.json({ results: records.filter(filterFn) });
  });
};
