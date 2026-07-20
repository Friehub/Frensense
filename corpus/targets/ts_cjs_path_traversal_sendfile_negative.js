// SAFE: The file path is resolved and checked to stay within the allowed base directory.

const path = require('path');
const express = require('express');

const app = express();
var NOTES_DIR = path.resolve('/var/www/notes');

app.get('/api/notes/:filename', function(req, res) {
  var filePath = path.resolve(NOTES_DIR, req.params.filename);
  if (!filePath.startsWith(NOTES_DIR)) {
    return res.status(400).send('Invalid path');
  }
  res.sendFile(filePath);
});
