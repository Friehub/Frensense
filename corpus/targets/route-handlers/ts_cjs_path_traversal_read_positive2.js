// [frensense]
// observation: User-controlled filename flows through an intermediate variable into fs.readFileSync without path sanitization.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences (e.g., ../../../etc/passwd).
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory.

var fs = require('fs');
var path = require('path');
var express = require('express');
var app = express();

function serveUserFile(req, res) {
  var base = '/data/user_content/';
  var fileName = req.params.file;
  var fullPath = base + fileName;
  var content = fs.readFileSync(fullPath);
  res.send(content);
}

function loadConfig(req, res) {
  var cfgPath = req.query.config;
  var data = fs.readFileSync('/etc/app/' + cfgPath, 'utf-8');
  res.type('json').send(data);
}

app.get('/files/:file', serveUserFile);
app.get('/admin/config', loadConfig);
