// SAFE: The user input is validated as a path segment and concatenated to a fixed base URL, preventing host injection.

const http = require('http');
const express = require('express');

const app = express();

app.get('/api/data', function(req, res) {
  const endpoint = req.query.endpoint;
  if (!/^[a-zA-Z0-9\/\-_]+$/.test(endpoint)) {
    return res.status(400).send('Invalid path');
  }
  http.get('http://api.example.com/' + endpoint, function(response) {
    var data = '';
    response.on('data', function(chunk) { data += chunk; });
    response.on('end', function() { res.send(data); });
  });
});
