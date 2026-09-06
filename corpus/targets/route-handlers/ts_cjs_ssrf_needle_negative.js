// SAFE: The URL is validated against an allowlist of permitted hosts before the needle.get() call, preventing SSRF to internal services.

const needle = require('needle');
const express = require('express');
const url = require('url');

const app = express();
const ALLOWED_HOSTS = ['api.example.com', 'data.example.com'];

app.get('/fetch', function(req, res) {
  const userUrl = req.query.url;
  const parsed = url.parse(userUrl);
  if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
    return res.status(400).send('Disallowed host');
  }
  needle.get(userUrl, function(err, response) {
    if (err) return res.status(500).send('Error');
    res.send(response.body);
  });
});
