// SAFE: Only local file paths are served, with strict directory traversal protection.

const path = require('path');
const express = require('express');

const app = express();
var UPLOADS_DIR = path.join(__dirname, 'uploads');

app.get('/api/download', function(req, res) {
  var safePath = path.join(UPLOADS_DIR, path.basename(req.query.file));
  if (!safePath.startsWith(UPLOADS_DIR)) {
    return res.status(400).send('Invalid path');
  }
  res.sendFile(safePath);
});
