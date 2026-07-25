// [frensense]
// observation: User-controlled filename flows through an intermediate variable into fs.readFileSync without path sanitization.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences (e.g., ../../../etc/passwd).
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/uploads", filename);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const assetPath = req.query.path;
    const fullPath = path.join("/var/static", assetPath);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
