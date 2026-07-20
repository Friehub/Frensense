// SAFE: HTTPS is used instead of HTTP, encrypting all data transmitted to the external API.

const https = require('https');
const express = require('express');

const app = express();

app.get('/api/external-data', function(req, res) {
  https.get('https://api.example.com/data', function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
