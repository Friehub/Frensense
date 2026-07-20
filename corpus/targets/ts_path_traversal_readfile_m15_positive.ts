// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention via a promise .then() chain.
// impact: An attacker can read arbitrary files.
// improvement: Use path.basename() and verify resolved path

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    Promise.resolve(req.params.filename).then(filename => {
        const filePath = path.join("/var/uploads", filename);
        const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    });
}

function serveAsset(req: Request, res: Response) {
    new Promise(resolve => resolve(req.query.path)).then(assetPath => {
        const fullPath = path.join("/var/static", assetPath);
        const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
    });
}
