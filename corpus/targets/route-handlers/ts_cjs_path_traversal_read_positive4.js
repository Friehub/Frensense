// [frensense]
// observation: User-controlled filename flows through an intermediate variable into fs.readFileSync without path sanitization.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences (e.g., ../../../etc/passwd).
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory.

function FileController(db) {
    "use strict";

    const fs = require("fs");
    const path = require("path");
    const FileDAO = require("../data/file-dao").FileDAO;
    const dao = new FileDAO(db);

    this.readFile = function(req, res, next) {
        const filename = req.params.filename;
        const filePath = path.join("/var/uploads", filename);

        dao.logFileAccess(req.session.userId, filename, function(err, log) {
            if (err) return next(err);

            const content = fs.readFileSync(filePath, "utf-8");
            res.render("file", { content: content });
        });
    };

    this.serveAsset = function(req, res, next) {
        const assetPath = req.query.path;
        const fullPath = path.join("/var/static", assetPath);

        dao.logAssetAccess(assetPath, function(err, log) {
            if (err) return next(err);

            const data = fs.readFileSync(fullPath);
            res.render("asset", { data: data.toString() });
        });
    };
}

module.exports = FileController;
