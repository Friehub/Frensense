// [frensense]
// observation: User-controlled input from a query parameter is directly interpolated into the HTML response body without escaping or sanitization.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: Encode all user input before embedding it in HTML output, or use a template engine with auto-escaping.

var express = require('express');
var app = express();

function errorDisplay(req, res) {
  var message = req.query.msg;
  switch (req.query.code) {
    case '404':
      res.status(404).send('<h1>Not Found</h1><p>' + message + '</p>');
      break;
    case '500':
      res.status(500).send('<h1>Server Error</h1><pre>' + message + '</pre>');
      break;
    default:
      res.send('<html><body>' + message + '</body></html>');
  }
}

function commentPreview(req, res) {
  var comment = req.body.comment || '';
  res.send('<div class="preview">' + comment + '</div>');
}

app.get('/error', errorDisplay);
app.post('/preview', commentPreview);
