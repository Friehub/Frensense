// SAFE: User input is escaped before being passed to template rendering

var express = require('express');
var escapeHtml = require('escape-html');

module.exports = function(app, db) {
  app.get('/greeting', function(req, res) {
    var name = escapeHtml(req.query.name || 'Guest');
    res.render('greeting', { name: name });
  });

  app.post('/comment', function(req, res) {
    var comment = escapeHtml(req.body.comment);
    db.collection('comments').insertOne({
      text: comment,
      userId: req.session.userId,
      createdAt: new Date()
    }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Failed to post comment' });
      res.render('comment', { comment: comment });
    });
  });
};
