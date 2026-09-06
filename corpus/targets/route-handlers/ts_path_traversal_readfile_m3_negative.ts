import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const a = req.params.filename;
    const b = a;
    const safeName = path.basename(b);
    const filePath = path.join("/var/uploads", safeName);
    if (!filePath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const raw = req.query.path;
    const assetPath = raw;
    const normalized = path.normalize(assetPath);
    if (normalized.startsWith("..") || path.isAbsolute(normalized)) {
        return res.status(403).send("Forbidden");
    }
    const fullPath = path.join("/var/static", normalized);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
