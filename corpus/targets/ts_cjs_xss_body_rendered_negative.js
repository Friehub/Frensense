// SAFE: HTML-escaped body fields rendered in response

var express = require('express');
var escapeHtml = require('escape-html');

module.exports = function(app, db) {
  app.post('/submit-feedback', function(req, res) {
    var message = escapeHtml(req.body.message);
    var name = escapeHtml(req.body.name);

    db.collection('feedback').insertOne({
      message: req.body.message,
      name: req.body.name,
      userId: req.session.userId
    }, function(err, result) {
      if (err) return res.status(500).send('Error saving feedback');
      res.send('<html><body><h1>Thank you, ' + name + '</h1><p>' + message + '</p></body></html>');
    });
  });

  app.get('/feedback/:id', function(req, res) {
    db.collection('feedback').findOne({ _id: req.params.id }, function(err, feedback) {
      if (err) return res.status(500).send('Error');
      res.send('<div class="feedback">' + escapeHtml(feedback.message) + '</div>');
    });
  });
};
