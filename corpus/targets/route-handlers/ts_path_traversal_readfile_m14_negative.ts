// SAFE: Renamed variables with path sanitization
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = "/var/uploads";

function readFile(req: Request, res: Response) {
    const fileNameFromUser = req.params.filename;
    const safeName = path.basename(fileNameFromUser); const filePath = path.join(BASE_DIR, safeName);
    if (!filePath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
    const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const assetPathFromQuery = req.query.path;
    const safeName = path.basename(assetPathFromQuery); const fullPath = path.join(BASE_DIR, safeName);
    if (!fullPath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
    const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
}
