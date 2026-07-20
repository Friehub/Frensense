// SAFE: Uses sanitize-filename to strip path traversal
var sanitize = require('sanitize-filename');
var fs = require('fs');
var path = require('path');

function readFile(req, res) {
    var filename = sanitize(req.params.filename);
    var filePath = path.join("/var/uploads", filename);
    var content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req, res) {
    var assetPath = sanitize(req.query.path);
    var fullPath = path.join("/var/static", assetPath);
    var data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
