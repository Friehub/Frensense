// SAFE: The URL is checked against an allowlist and the deprecated request library is used with strict host validation, preventing SSRF.

const request = require('request');
const express = require('express');
const url = require('url');

const app = express();
const ALLOWED = ['api.example.com', 'cdn.example.com'];

app.get('/proxy', function(req, res) {
  const target = req.query.target;
  const parsed = url.parse(target);
  if (!ALLOWED.includes(parsed.hostname)) {
    return res.status(400).send('Disallowed');
  }
  request(target, function(err, response, body) {
    if (err) return res.status(500).send('Error');
    res.send(body);
  });
});
