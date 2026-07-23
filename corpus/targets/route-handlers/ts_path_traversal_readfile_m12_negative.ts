// SAFE: Try-catch with path sanitization
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = "/var/uploads";

function readFile(req: Request, res: Response) {
    try { const safeName = path.basename(req.params.filename); const filePath = path.join(BASE_DIR, safeName); if (!filePath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path"); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); } catch (err) { console.error(err); res.status(500).send("Error"); }
}

function serveAsset(req: Request, res: Response) {
    try { const safeName = path.basename(req.query.path); const fullPath = path.join(BASE_DIR, safeName); if (!fullPath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path"); const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data); } catch (err) { console.error(err); res.status(500).send("Error"); }
}
