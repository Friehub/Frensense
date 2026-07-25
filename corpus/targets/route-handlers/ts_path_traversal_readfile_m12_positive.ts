// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention inside a try-catch block.
// impact: An attacker can read arbitrary files, with errors silently caught.
// improvement: Use path.basename() and verify resolved path
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    try { const filePath = path.join("/var/uploads", req.params.filename); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); } catch (err) { console.error(err); }
}

function serveAsset(req: Request, res: Response) {
    try { const fullPath = path.join("/var/static", req.query.path); const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data); } catch {}
}
