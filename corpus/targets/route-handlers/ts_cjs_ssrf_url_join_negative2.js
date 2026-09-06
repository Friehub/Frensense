// SAFE: The full URL is parsed and the hostname is validated against an allowlist before making the request.

const url = require('url');
const http = require('http');
const express = require('express');

const app = express();
const ALLOWED_HOSTS = ['api.example.com'];

app.get('/api/data', function(req, res) {
  const fullUrl = 'http://api.example.com/' + (req.query.endpoint || '');
  const parsed = url.parse(fullUrl);
  if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
    return res.status(400).send('Disallowed host');
  }
  http.get(fullUrl, function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
