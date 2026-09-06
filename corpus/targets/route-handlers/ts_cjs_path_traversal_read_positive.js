// [frensense]
// observation: User-controlled filename flows through an intermediate variable into fs.readFileSync without path sanitization.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences (e.g., ../../../etc/passwd).
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory.

var fs = require('fs');
var path = require('path');

function readFile(req, res) {
    var filename = req.params.filename;
    var filePath = path.join("/var/uploads", filename);
    var content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req, res) {
    var assetPath = req.query.path;
    var fullPath = path.join("/var/static", assetPath);
    var data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
