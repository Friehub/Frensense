// [frensense]
// observation: The deprecated request() library is called with a user-supplied URL, allowing SSRF to internal networks without any validation.
// impact: An attacker can redirect the server to make requests to internal services like Redis, Memcached, or cloud metadata endpoints, leaking sensitive data.
// improvement: Replace with a modern HTTP client (e.g., node-fetch, undici) and validate the URL against an allowlist.

const request = require('request');
const express = require('express');

const app = express();

app.get('/proxy', function(req, res) {
  const target = req.query.target;
  request(target, function(err, response, body) {
    if (err) return res.status(500).send('Error');
    res.send(body);
  });
});
