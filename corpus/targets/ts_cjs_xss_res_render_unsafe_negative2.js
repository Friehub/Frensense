// SAFE alternative: Use a context-aware sanitizer like DOMPurify on server side

var express = require('express');
var createDOMPurify = require('dompurify');
var jsdom = require('jsdom');
var window = new jsdom.JSDOM('').window;
var DOMPurify = createDOMPurify(window);

module.exports = function(app, db) {
  app.get('/greeting', function(req, res) {
    var name = req.query.name || 'Guest';
    var cleanName = DOMPurify.sanitize(name);
    res.render('greeting', { name: cleanName });
  });

  app.post('/comment', function(req, res) {
    var cleanComment = DOMPurify.sanitize(req.body.comment);
    db.collection('comments').insertOne({
      text: cleanComment,
      userId: req.session.userId,
      createdAt: new Date()
    }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Failed to post comment' });
      res.render('comment', { comment: cleanComment });
    });
  });
};
