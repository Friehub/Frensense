// [frensense]
// observation: User-controlled filename is passed through a helper function that returns the path without sanitization before fs.readFileSync.
// impact: An attacker can read arbitrary files by supplying path traversal sequences through an unsafe helper.
// improvement: Apply path.basename() inside the helper to strip directory components.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import * as fs from "fs";
import * as path from "path";

function resolvePath(base: string, userInput: string): string {
    return path.join(base, userInput);
}

function readFile(req: Request, res: Response) {
    const filePath = resolvePath("/var/uploads", req.params.filename);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const fullPath = resolvePath("/var/static", req.query.path);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
