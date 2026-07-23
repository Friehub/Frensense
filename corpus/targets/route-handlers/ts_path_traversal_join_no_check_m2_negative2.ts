// SAFE: Uses path.basename to strip directory traversal
import * as fs from "fs"; import * as path from "path";
function handlerA(req: Request, res: Response) {
    const filename = path.basename(req.params.filename);
    const filePath = path.join("/var/uploads", filename);
    const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
}
function handlerB(req: Request, res: Response) {
    const safePath = path.basename(req.query.path);
    const filePath = path.join("/var/uploads", safePath);
    const content = fs.readFileSync(filePath); res.type("application/octet-stream").send(content);
}
