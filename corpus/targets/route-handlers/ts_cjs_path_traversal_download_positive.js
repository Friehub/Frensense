// [frensense]
// observation: res.download() is called with a user-supplied file path from req.params.file without any validation, allowing an attacker to download arbitrary files from the server.
// impact: An attacker can traverse directories with ../../etc/passwd to download sensitive files including configuration, source code, SSH keys, and credentials.
// improvement: Validate that the resolved path stays within an allowed base directory using path.resolve() and path.startsWith().

const path = require('path');
const express = require('express');

const app = express();

app.get('/api/files/:file', function(req, res) {
  var filePath = path.join('/var/www/uploads', req.params.file);
  res.download(filePath);
});
