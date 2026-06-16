import * as fs from "fs";
import * as path from "path";

function readFile(req: Request, res: Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/uploads", filename);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

function serveAsset(req: Request, res: Response) {
    const assetPath = req.query.path;
    const fullPath = path.join("/var/static", assetPath);
    const data = fs.readFileSync(fullPath);
    res.type("application/octet-stream").send(data);
}

function downloadFile(req: Request, res: Response) {
    const name = req.body.name;
    const file = fs.readFileSync(`/data/${name}`);
    res.send(file);
}
