// [frensense]
// observation: The server uses http.get() or http.request() for external API calls without TLS, transmitting data in cleartext over the network.
// impact: An attacker on the network path (MITM) can intercept, read, and modify sensitive data exchanged with the external API, including API keys, tokens, and personal information.
// improvement: Use https.get() or https.request() with proper TLS configuration to encrypt data in transit.

const http = require('http');
const express = require('express');

const app = express();

app.get('/api/external-data', function(req, res) {
  http.get('http://api.example.com/data', function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
