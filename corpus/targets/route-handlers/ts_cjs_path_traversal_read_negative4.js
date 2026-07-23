// SAFE: Strips ../ sequences and validates against an allowlist

var fs = require('fs');
var path = require('path');

var ALLOWED_PREFIXES = ['notes/', 'docs/', 'public/'];

function sanitizePath(input) {
  var normalized = path.normalize(input).replace(/\.\.\//g, '');
  while (normalized.indexOf('../') !== -1) {
    normalized = normalized.replace('../', '');
  }
  return normalized;
}

function isAllowed(filePath) {
  return ALLOWED_PREFIXES.some(function(prefix) {
    return filePath.indexOf(prefix) === 0;
  });
}

function readDocument(req, res) {
  var rawPath = req.query.path || '';
  var safePath = sanitizePath(rawPath);
  if (!isAllowed(safePath)) {
    return res.status(403).send('Path not allowed');
  }
  var fullPath = path.join('/var/storage', safePath);
  fs.readFile(fullPath, function(err, data) {
    if (err) return res.status(404).send('Not found');
    res.send(data);
  });
}

function readAsset(req, res) {
  var rawPath = req.params.file || '';
  var safePath = sanitizePath(rawPath);
  if (!isAllowed(safePath)) {
    return res.status(403).send('Path not allowed');
  }
  var fullPath = path.join('/var/storage', safePath);
  fs.createReadStream(fullPath).pipe(res);
}
