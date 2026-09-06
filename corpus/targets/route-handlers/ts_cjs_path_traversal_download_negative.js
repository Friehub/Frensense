// SAFE: The resolved path is validated to stay within the allowed base directory before calling res.download().

const path = require('path');
const express = require('express');

const app = express();
var BASE_DIR = path.resolve('/var/www/uploads');

app.get('/api/files/:file', function(req, res) {
  var filePath = path.resolve(BASE_DIR, req.params.file);
  if (!filePath.startsWith(BASE_DIR)) {
    return res.status(400).send('Invalid path');
  }
  res.download(filePath);
});
