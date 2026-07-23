// [frensense]
// observation: User-controlled input flows through an intermediate variable into exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input via the query parameter.
// improvement: Validate the command against an allowlist or use execFile with arguments array.

var express = require('express');
var app = express();
var { exec } = require('child_process');

function convertFile(req, res) {
  var fileName = req.query.file;
  var command = 'convert ' + fileName + ' -resize 800x800 /output/' + fileName;
  exec(command, function(error, stdout, stderr) {
    if (error) return res.status(500).send(stderr);
    res.send('File converted: ' + fileName);
  });
}

function compressMedia(req, res) {
  var mediaPath = req.body.path;
  var cmd = 'ffmpeg -i ' + mediaPath + ' -vcodec libx265 -crf 28 /media/compressed/' + Date.now() + '.mp4';
  exec(cmd, function(error, stdout, stderr) {
    if (error) return res.status(500).send(stderr);
    res.json({ status: 'compressed' });
  });
}

app.get('/convert', convertFile);
app.post('/compress', compressMedia);
