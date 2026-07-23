// SAFE: User input passed only to path.basename() for safe file operations

var fs = require('fs');
var path = require('path');

function readUserFile(req, res) {
  var userInput = req.query.file;
  var safeName = path.basename(userInput);
  var filePath = path.join('/var/data', safeName);
  fs.readFile(filePath, 'utf-8', function(err, content) {
    if (err) return res.status(500).send('Error reading file');
    res.send(content);
  });
}

function writeUserLog(req, res) {
  var logName = req.body.log;
  var safeLog = path.basename(logName);
  var logPath = path.join('/var/logs', safeLog);
  fs.appendFile(logPath, req.body.data + '\n', function(err) {
    if (err) return res.status(500).send('Error writing log');
    res.send({ success: true });
  });
}
