// SAFE: Only basename is used for security, preventing any directory traversal in the user input.

const path = require('path');
const express = require('express');

const app = express();
var NOTES_DIR = '/var/www/notes';

app.get('/api/notes/:filename', function(req, res) {
  var filename = path.basename(req.params.filename);
  var filePath = path.join(NOTES_DIR, filename);
  res.sendFile(filePath);
});
