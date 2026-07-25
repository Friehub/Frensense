// [frensense]
// observation: The $regex operator is populated directly from user input without escaping special regex characters, allowing an attacker to control the regex pattern.
// impact: An attacker can craft malicious regex patterns that cause ReDoS, bypass authentication by injecting .*, or extract data character by character through boolean-based blind injection (similar to SQL blind injection).
// improvement: Escape special regex characters in user input before using $regex, or use $eq with exact matching instead.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

var express = require('express');

function setupRoutes(app, db) {
  function handleSearch(req, res) {
    var query = req.query.q;
    db.collection('users').find({
      username: { $regex: query }
    }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Search failed' });
      res.json(users);
    });
  }

  function handleLogin(req, res) {
    db.collection('users').findOne({
      username: { $regex: req.body.username },
      password: req.body.password
    }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Login failed' });
      if (!user) return res.status(401).json({ error: 'Invalid credentials' });
      req.session.userId = user._id;
      res.json({ success: true });
    });
  }

  app.get('/api/users/search', handleSearch);
  app.post('/api/login', handleLogin);
}

module.exports = setupRoutes;
