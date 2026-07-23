// SAFE: Async path with path sanitization
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = "/var/uploads";

async function getFilename(req: any): Promise<string> {
    const name = req.params.filename;
    const safe = path.basename(name); const resolved = path.join(BASE_DIR, safe);
    if (!resolved.startsWith(BASE_DIR)) throw new Error("Invalid path");
    return resolved;
}

async function getAssetPath(req: any): Promise<string> {
    const p = req.query.path; const safe = path.basename(p); const resolved = path.join(BASE_DIR, safe);
    if (!resolved.startsWith(BASE_DIR)) throw new Error("Invalid path"); return resolved;
}

async function readFile(req: Request, res: Response) {
    try { const filePath = await getFilename(req); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); } catch { res.status(403).send("Invalid path"); }
}

async function serveAsset(req: Request, res: Response) {
    try { const fullPath = await getAssetPath(req); const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data); } catch { res.status(403).send("Invalid path"); }
}
