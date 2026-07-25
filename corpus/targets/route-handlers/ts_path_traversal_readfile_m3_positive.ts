// [frensense]
// observation: User-controlled filename traverses multiple variable assignments before reaching fs.readFileSync without sanitization.
// impact: An attacker can read arbitrary files by supplying path traversal sequences through multi-hop assignments.
// improvement: Use path.basename() on the final value before constructing the file path.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const a = req.params.filename;
    const b = a;
    const filePath = path.join("/var/uploads", b);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const raw = req.query.path;
    const assetPath = raw;
    const fullPath = path.join("/var/static", assetPath);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
