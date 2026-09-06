// SAFE: Only the filename (basename) is used, stripping any directory traversal components.

const path = require('path');
const express = require('express');

const app = express();
var BASE_DIR = '/var/www/uploads';

app.get('/api/files/:file', function(req, res) {
  var filename = path.basename(req.params.file);
  var filePath = path.join(BASE_DIR, filename);
  res.download(filePath);
});
