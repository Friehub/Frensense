import * as fs from "fs";
import * as path from "path";

function resolveSafePath(base: string, userInput: string): string {
    const safe = path.basename(userInput);
    const resolved = path.join(base, safe);
    if (!resolved.startsWith(base)) {
        throw new Error("Path traversal detected");
    }
    return resolved;
}

function readFile(req: Request, res: Response) {
    try {
        const filePath = resolveSafePath("/var/uploads", req.params.filename);
        const content = fs.readFileSync(filePath, "utf-8");
        res.send(content);
    } catch {
        res.status(403).send("Forbidden");
    }
}

function serveAsset(req: Request, res: Response) {
    try {
        const fullPath = resolveSafePath("/var/static", req.query.path);
        const data = fs.readFileSync(fullPath);
        res.type("application/octet-stream").send(data);
    } catch {
        res.status(403).send("Forbidden");
    }
}
