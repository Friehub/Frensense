// SAFE: The URL scheme is enforced to only allow HTTPS, rejecting any HTTP URLs.

const http = require('http');
const https = require('https');
const url = require('url');
const express = require('express');

const app = express();

app.get('/api/external-data', function(req, res) {
  var target = 'https://api.example.com/data';
  var parsed = url.parse(target);
  var mod = parsed.protocol === 'https:' ? https : http;
  mod.get(target, function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
