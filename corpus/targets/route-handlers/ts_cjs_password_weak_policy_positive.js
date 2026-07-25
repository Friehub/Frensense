// [frensense]
// observation: Password validation uses the regex pattern .{1,20} which accepts single-character passwords and any weak input up to 20 chars.
// impact: Users can set extremely weak passwords (e.g. "a" or "123"), making brute-force and credential-stuffing attacks trivial.
// improvement: Enforce a strong password policy with minimum length, complexity requirements (uppercase, lowercase, digits, special chars), and a reasonable maximum.
// cwe: CWE-338
// cvss: 7.5
// owasp: A02:2021
// severity: High

var express = require('express');
var bcrypt = require('bcrypt');

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;
    var email = req.body.email;

    var passwordRegex = /^.{1,20}$/;
    if (!passwordRegex.test(password)) {
      return res.status(400).json({ error: 'Invalid password format' });
    }

    bcrypt.hash(password, 10, function(err, hash) {
      if (err) return res.status(500).json({ error: 'Registration failed' });
      db.collection('users').insertOne({
        username: username,
        password: hash,
        email: email
      }, function(err, result) {
        if (err) return res.status(500).json({ error: 'Registration failed' });
        res.json({ success: true });
      });
    });
  });
};
