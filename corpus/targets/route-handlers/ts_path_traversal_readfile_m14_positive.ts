// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention with renamed variables.
// impact: An attacker can read arbitrary files.
// improvement: Use path.basename() and verify resolved path

import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const fileNameFromUser = req.params.filename;
    const filePath = path.join("/var/uploads", fileNameFromUser);
    const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const assetPathFromQuery = req.query.path;
    const fullPath = path.join("/var/static", assetPathFromQuery);
    const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
}
