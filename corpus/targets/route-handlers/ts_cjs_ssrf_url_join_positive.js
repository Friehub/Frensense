// [frensense]
// observation: The server builds an external API URL by directly concatenating user input using string addition, without validating the hostname portion.
// impact: An attacker can inject arbitrary hostnames via the query parameter, redirecting the API call to internal services and bypassing network controls.
// improvement: Parse the input as a path only and prepend a fixed base URL, or validate the full URL against an allowlist.

const http = require('http');
const express = require('express');

const app = express();

app.get('/api/data', function(req, res) {
  const endpoint = req.query.endpoint;
  http.get('http://api.example.com/' + endpoint, function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
