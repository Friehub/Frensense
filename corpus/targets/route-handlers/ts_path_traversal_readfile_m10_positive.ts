// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention across an async/await boundary.
// impact: An attacker can read arbitrary files.
// improvement: Use path.basename() and verify the resolved path stays within allowed directory

import * as fs from "fs";
import * as path from "path";

async function getFilename(req: any): Promise<string> { return req.params.filename; }
async function getAssetPath(req: any): Promise<string> { return req.query.path; }

async function readFile(req: Request, res: Response) {
    const filename = await getFilename(req);
    const filePath = path.join("/var/uploads", filename);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

async function serveAsset(req: Request, res: Response) {
    const assetPath = await getAssetPath(req);
    const fullPath = path.join("/var/static", assetPath);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
