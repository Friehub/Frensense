// SAFE: Conditional branch with path sanitization
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = "/var/uploads";

function readFile(req: Request, res: Response) {
    if (req.params.filename) {
        const safeName = path.basename(req.params.filename);
        const filePath = path.join(BASE_DIR, safeName);
        if (!filePath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
        const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    } else { res.status(400).send("Missing filename"); }
}

function serveAsset(req: Request, res: Response) {
    if (req.query.path && req.query.path.length > 0) {
        const safeName = path.basename(req.query.path);
        const fullPath = path.join(BASE_DIR, safeName);
        if (!fullPath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
        const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
    }
}
