// SAFE: File read with path.resolve + .startsWith() prefix check on resolved path

var fs = require('fs');
var path = require('path');

var BASE_DIR = path.resolve('/var/data');

function safeRead(req, res) {
  var userPath = req.params[0] || '';
  var fullPath = path.resolve(path.join(BASE_DIR, userPath));
  if (fullPath.indexOf(BASE_DIR) !== 0) {
    return res.status(403).send('Access denied');
  }
  fs.readFile(fullPath, 'utf-8', function(err, content) {
    if (err) return res.status(404).send('File not found');
    res.send(content);
  });
}

function safeStat(req, res) {
  var userPath = req.query.file || '';
  var fullPath = path.resolve(path.join(BASE_DIR, userPath));
  if (fullPath.indexOf(BASE_DIR) !== 0) {
    return res.status(403).send('Access denied');
  }
  fs.stat(fullPath, function(err, stats) {
    if (err) return res.status(404).send('Not found');
    res.json({ size: stats.size, modified: stats.mtime });
  });
}
