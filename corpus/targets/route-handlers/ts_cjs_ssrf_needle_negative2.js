// SAFE: Only relative paths are fetched by prepending a fixed base URL, eliminating the risk of SSRF to arbitrary hosts.

const needle = require('needle');
const express = require('express');

const app = express();
const BASE_URL = 'https://api.example.com';

app.get('/fetch', function(req, res) {
  const path = req.query.path;
  needle.get(BASE_URL + '/' + path.replace(/^\/+/, ''), function(err, response) {
    if (err) return res.status(500).send('Error');
    res.send(response.body);
  });
});
