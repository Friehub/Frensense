// SAFE: Object property path sanitized with path.basename
import * as fs from "fs";
import * as path from "path";

const BASE_DIR = "/var/uploads";

function readFile(req: Request, res: Response) {
    const cfg = { name: req.params.filename };
    const safeName = path.basename(cfg.name);
    const filePath = path.join(BASE_DIR, safeName);
    if (!filePath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const opts = { p: req.query.path };
    const safeName = path.basename(opts.p);
    const fullPath = path.join(BASE_DIR, safeName);
    if (!fullPath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}
