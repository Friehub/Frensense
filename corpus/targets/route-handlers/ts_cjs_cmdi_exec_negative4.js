// SAFE: User-controlled filename validated against an allowlist before any operation

var fs = require('fs');
var path = require('path');

var ALLOWED_FILES = ['report.csv', 'export.json', 'backup.zip', 'config.yml'];

function validateFile(filename) {
  return ALLOWED_FILES.indexOf(filename) !== -1;
}

function exportReport(req, res) {
  var fileName = req.query.file;
  if (!validateFile(fileName)) {
    return res.status(403).send('File not allowed');
  }
  var filePath = path.join('/var/exports', fileName);
  fs.readFile(filePath, function(err, data) {
    if (err) return res.status(500).send('Error reading file');
    res.send(data);
  });
}

function deleteReport(req, res) {
  var fileName = req.body.file;
  if (!validateFile(fileName)) {
    return res.status(403).send('File not allowed');
  }
  var filePath = path.join('/var/exports', fileName);
  fs.unlink(filePath, function(err) {
    if (err) return res.status(500).send('Error deleting file');
    res.send({ success: true });
  });
}
