// SAFE: FTP is replaced with HTTPS download, encrypting data in transit.

const https = require('https');
const express = require('express');

const app = express();

app.get('/api/download', function(req, res) {
  var allowedHosts = ['cdn.example.com', 'files.example.com'];
  if (!allowedHosts.includes(req.query.host)) {
    return res.status(400).send('Disallowed host');
  }
  https.get('https://' + req.query.host + '/' + req.query.path, function(response) {
    response.pipe(res);
  });
});
