// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention through an object property.
// impact: An attacker can read arbitrary files on the server by supplying path traversal sequences.
// improvement: Use path.basename() to strip directory components and verify the resolved path stays within the allowed directory

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const cfg = { name: req.params.filename };
    const filePath = path.join("/var/uploads", cfg.name);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const opts = { p: req.query.path };
    const fullPath = path.join("/var/static", opts.p);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
