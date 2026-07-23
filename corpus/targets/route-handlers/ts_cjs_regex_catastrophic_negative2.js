// SAFE alternative: Use re2 or set a timeout for regex execution

var express = require('express');
var RE2 = require('re2');

module.exports = function(app, db) {
  app.post('/validate-code', function(req, res) {
    var code = req.body.code;
    var regex = new RE2(/^[0-9]+#$/);

    if (regex.test(code)) {
      res.json({ valid: true });
    } else {
      res.json({ valid: false });
    }
  });

  app.post('/search', function(req, res) {
    var query = req.body.query;
    var pattern = new RE2(/^[a-zA-Z]+(?:\s+[a-zA-Z]+)*$/i);

    db.collection('items').find({ name: { $regex: query } }).toArray(function(err, items) {
      res.json({ results: items });
    });
  });
};
