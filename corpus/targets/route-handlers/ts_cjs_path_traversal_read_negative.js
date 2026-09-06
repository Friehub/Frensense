var fs = require('fs');
var path = require('path');

function readFile(req, res) {
    var filename = req.params.filename;
    var safeName = path.basename(filename);
    var filePath = path.join("/var/uploads", safeName);
    if (filePath.indexOf("/var/uploads") !== 0) {
        return res.status(403).send("Forbidden");
    }
    var content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req, res) {
    var assetPath = req.query.path;
    var normalized = path.normalize(assetPath);
    if (normalized.indexOf("..") === 0 || path.isAbsolute(normalized)) {
        return res.status(403).send("Forbidden");
    }
    var fullPath = path.join("/var/static", normalized);
    if (fullPath.indexOf("/var/static") !== 0) {
        return res.status(403).send("Forbidden");
    }
    var data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
