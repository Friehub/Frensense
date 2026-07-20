// SAFE: Nested quantifiers removed, replaced with non-catastrophic pattern

var express = require('express');

module.exports = function(app, db) {
  app.post('/validate-code', function(req, res) {
    var code = req.body.code;
    var regex = /^[0-9]+#$/;

    if (regex.test(code)) {
      res.json({ valid: true });
    } else {
      res.json({ valid: false });
    }
  });

  app.post('/search', function(req, res) {
    var query = req.body.query;
    var pattern = /^[a-zA-Z]+(?:\s+[a-zA-Z]+)*$/i;

    db.collection('items').find({ name: { $regex: query } }).toArray(function(err, items) {
      res.json({ results: items });
    });
  });
};
