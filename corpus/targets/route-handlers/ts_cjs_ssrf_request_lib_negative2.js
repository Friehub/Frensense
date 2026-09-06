// SAFE: Migrated to node-fetch with a validated base URL, preventing arbitrary URL injection.

const fetch = require('node-fetch');
const express = require('express');

const app = express();
const BASE = 'https://api.example.com';

app.get('/proxy', function(req, res) {
  const path = req.query.path;
  const safePath = path.replace(/[^a-zA-Z0-9\-_.\/]/g, '');
  fetch(BASE + '/' + safePath)
    .then(function(response) { return response.text(); })
    .then(function(body) { res.send(body); })
    .catch(function() { res.status(500).send('Error'); });
});
