// [frensense]
// observation: User-controlled filename flows through an intermediate variable into fs.readFileSync without path sanitization.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences (e.g., ../../../etc/passwd).
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory.

var fs = require('fs');
var express = require('express');
var app = express();

function downloadDocument(req, res) {
  var userPath = req.params.document;
  if (!userPath) {
    return res.status(400).send('Missing document name');
  }
  var stream = fs.createReadStream(userPath);
  stream.on('error', function(err) {
    res.status(500).send('Cannot read file');
  });
  stream.pipe(res);
}

function viewSource(req, res) {
  var script = req.query.script;
  var src = fs.readFileSync('/var/www/src/' + script, 'utf-8');
  res.send('<pre>' + src + '</pre>');
}

app.get('/docs/:document', downloadDocument);
app.get('/debug/source', viewSource);
