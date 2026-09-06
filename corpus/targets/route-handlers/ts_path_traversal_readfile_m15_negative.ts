// SAFE: .then() chain with path sanitization
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = "/var/uploads";

function readFile(req: Request, res: Response) {
    Promise.resolve(req.params.filename).then(filename => {
        const safeName = path.basename(filename); const filePath = path.join(BASE_DIR, safeName);
        if (!filePath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
        const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    });
}

function serveAsset(req: Request, res: Response) {
    new Promise(resolve => resolve(req.query.path)).then(assetPath => {
        const safeName = path.basename(assetPath); const fullPath = path.join(BASE_DIR, safeName);
        if (!fullPath.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
        const data = fs.readFileSync(fullPath); res.type("application/octet-stream").send(data);
    });
}
