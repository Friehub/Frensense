// [frensense]
// observation: res.sendFile() is called with a user-supplied path constructed from URL parameters without any path validation, enabling directory traversal.
// impact: An attacker can read arbitrary files from the filesystem, including /etc/passwd, application source code, configuration files containing secrets, and database credentials.
// improvement: Validate the resolved path against an allowed base directory using path.resolve() and ensure it starts with the allowed base path.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

const path = require('path');
const express = require('express');

const app = express();

app.get('/api/notes/:filename', function(req, res) {
  var filePath = path.join('/var/www/notes', req.params.filename);
  res.sendFile(filePath);
});
