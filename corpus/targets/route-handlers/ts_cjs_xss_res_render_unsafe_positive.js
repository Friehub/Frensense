// [frensense]
// observation: User input from query parameters or body is passed directly to res.render() as template data without escaping or sanitization.
// impact: If the template engine renders data unescaped (e.g. using <%- %> in EJS), an attacker can inject arbitrary JavaScript into the page, leading to stored or reflected XSS.
// improvement: Always escape user input before passing it to templates, or use template engines that auto-escape by default (e.g. EJS with <%= %>).
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

var express = require('express');

module.exports = function(app, db) {
  app.get('/greeting', function(req, res) {
    var name = req.query.name || 'Guest';
    res.render('greeting', { name: name });
  });

  app.post('/comment', function(req, res) {
    var comment = req.body.comment;
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
