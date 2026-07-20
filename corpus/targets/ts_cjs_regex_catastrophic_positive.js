// [frensense]
// observation: User input is validated against a regex with nested quantifiers (([0-9]+)+#) causing catastrophic backtracking on non-matching input.
// impact: An attacker can send a crafted string like "999999999999999999999" causing CPU-bound ReDoS (Regular Expression Denial of Service), making the server unresponsive.
// improvement: Remove nested quantifiers, use atomic groups, set a regex execution timeout, or use a simpler validation approach.

var express = require('express');

module.exports = function(app, db) {
  app.post('/validate-code', function(req, res) {
    var code = req.body.code;
    var regex = /^([0-9]+)+#$/;

    if (regex.test(code)) {
      res.json({ valid: true });
    } else {
      res.json({ valid: false });
    }
  });

  app.post('/search', function(req, res) {
    var query = req.body.query;
    var pattern = new RegExp('^([a-zA-Z]+\\s+)+$', 'i');

    db.collection('items').find({ name: { $regex: query } }).toArray(function(err, items) {
      if (pattern.test(query)) {
        res.json({ results: items });
      } else {
        res.json({ results: items });
      }
    });
  });
};
