// [frensense]
// observation: req.body.field is rendered directly in HTML response without any encoding, sanitization, or Content-Type header enforcement.
// impact: An attacker can submit HTML/JavaScript via form fields that gets executed in the browser of anyone viewing the response, enabling stored XSS attacks.
// improvement: Always escape HTML characters in user input before embedding in responses, and set the Content-Type header appropriately.

var express = require('express');

module.exports = function(app, db) {
  app.post('/submit-feedback', function(req, res) {
    var message = req.body.message;
    var name = req.body.name;

    db.collection('feedback').insertOne({
      message: message,
      name: name,
      userId: req.session.userId
    }, function(err, result) {
      if (err) return res.status(500).send('Error saving feedback');
      res.send('<html><body><h1>Thank you, ' + name + '</h1><p>' + message + '</p></body></html>');
    });
  });

  app.get('/feedback/:id', function(req, res) {
    db.collection('feedback').findOne({ _id: req.params.id }, function(err, feedback) {
      if (err) return res.status(500).send('Error');
      res.send('<div class="feedback">' + feedback.message + '</div>');
    });
  });
};
