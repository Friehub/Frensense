// SAFE alternative: Return JSON instead of HTML, letting client handle rendering safely

var express = require('express');

module.exports = function(app, db) {
  app.post('/submit-feedback', function(req, res) {
    db.collection('feedback').insertOne({
      message: req.body.message,
      name: req.body.name,
      userId: req.session.userId
    }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Error saving feedback' });
      res.json({
        success: true,
        feedback: {
          id: result.insertedId,
          name: req.body.name,
          message: req.body.message
        }
      });
    });
  });

  app.get('/feedback/:id', function(req, res) {
    db.collection('feedback').findOne({ _id: req.params.id }, function(err, feedback) {
      if (err) return res.status(500).json({ error: 'Error' });
      res.json({ feedback: feedback });
    });
  });
};
