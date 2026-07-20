// [frensense]
// observation: The application uses an FTP URL to download a file from a user-supplied path, transmitting data and credentials in cleartext over the network.
// impact: An attacker on the network can intercept FTP traffic including any credentials in the URL, and can also inject malicious files by exploiting the lack of TLS on the FTP connection.
// improvement: Use SFTP or FTPS for file transfers. If only HTTP/HTTPS sources are needed, validate the URL scheme and reject non-TLS protocols.

const fs = require('fs');
const ftp = require('ftp');
const express = require('express');

const app = express();

app.get('/api/download', function(req, res) {
  var client = new ftp();
  client.connect({ host: req.query.host, user: 'anonymous', password: 'anonymous' });
  client.get(req.query.path, function(err, stream) {
    if (err) return res.status(500).send('Download failed');
    stream.pipe(res);
  });
});
