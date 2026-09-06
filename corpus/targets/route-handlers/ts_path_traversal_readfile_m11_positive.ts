// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention inside a conditional block on the tainted branch.
// impact: An attacker can read arbitrary files.
// improvement: Use path.basename() and verify resolved path
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    if (req.params.filename) {
        const filePath = path.join("/var/uploads", req.params.filename);
        const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    } else { res.status(400).send("Missing filename"); }
}

function serveAsset(req: Request, res: Response) {
    if (req.query.path && req.query.path.length > 0) {
        const fullPath = path.join("/var/static", req.query.path);
        const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
    }
}
