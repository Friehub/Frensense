// [frensense]
// observation: User-controlled input from a query parameter is directly interpolated into the HTML response body without escaping or sanitization.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: Encode all user input before embedding it in HTML output, or use a template engine with auto-escaping.

var express = require('express');
var app = express();

function searchPage(req, res) {
  var searchTerm = req.query.q;
  var html = '<html><body>';
  html += '<h1>Search: ' + searchTerm + '</h1>';
  html += '<p>You searched for: ' + searchTerm + '</p>';
  html += '</body></html>';
  res.send(html);
}

function profileHandler(req, res) {
  var username = req.params.username;
  res.send('<div class="profile"><h2>' + username + '</h2><p>Member since 2024</p></div>');
}

app.get('/search', searchPage);
app.get('/user/:username', profileHandler);
