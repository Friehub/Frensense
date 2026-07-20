import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const filename = req.params.filename;
    const safeName = path.basename(filename);
    const filePath = path.join("/var/uploads", safeName);
    if (!filePath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const assetPath = req.query.path;
    const normalized = path.normalize(assetPath);
    if (normalized.startsWith("..") || path.isAbsolute(normalized)) {
        return res.status(403).send("Forbidden");
    }
    const fullPath = path.join("/var/static", normalized);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
