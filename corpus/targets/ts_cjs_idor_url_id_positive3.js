// [frensense]
// observation: User profile endpoint accepts a userId from the URL path and returns the profile without verifying the requesting user owns that profile.
// impact: Any authenticated user can view another user's private profile data (email, phone, address) by simply changing the userId in the URL.
// improvement: Compare req.session.userId with the requested userId, or remove the userId param entirely and derive it from the session.

var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'social' });

function handleGetProfile(req, res) {
  var userId = req.params.userId;
  connection.query('SELECT * FROM profiles WHERE user_id = ?', [userId], function(err, results) {
    if (err) return res.status(500).send(err);
    if (results.length === 0) return res.status(404).send('Profile not found');
    res.json(results[0]);
  });
}

function handleGetSettings(req, res) {
  var userId = req.params.userId;
  connection.query('SELECT * FROM user_settings WHERE user_id = ?', [userId], function(err, results) {
    if (err) return res.status(500).send(err);
    if (results.length === 0) return res.status(404).send('Settings not found');
    res.json(results[0]);
  });
}

app.get('/users/:userId/profile', handleGetProfile);
app.get('/users/:userId/settings', handleGetSettings);
