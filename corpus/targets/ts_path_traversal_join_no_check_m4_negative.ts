// SAFE: Implements safe alternative
// SAFE: Resolved path is verified against the intended base directory
import * as fs from "fs"; import * as path from "path";
const BASE = "/var/uploads";
function safePath(userPath: string): string {
    const resolved = path.resolve(BASE, userPath);
    if (!resolved.startsWith(BASE)) throw new Error("Invalid path");
    return resolved;
}
function handlerA(req: Request, res: Response) {
    try { const filePath = safePath(req.params.filename); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); }
    catch { res.status(403).send("Invalid path"); }
}
function handlerB(req: Request, res: Response) {
    try { const filePath = safePath(req.query.path); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); }
    catch { res.status(403).send("Invalid path"); }
}
