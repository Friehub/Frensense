import * as fs from "fs";
import * as path from "path";

function downloadFile(req: Request, res: Response) {
    const name = req.body.name;
    const safeName = path.basename(name);
    const filePath = path.join("/data", safeName);
    if (!filePath.startsWith("/data")) {
        return res.status(403).send("Forbidden");
    }
    const file = fs.readFileSync(filePath);
    res.send(file);
}

function readFile(req: Request, res: Response) {
    const safeName = path.basename(req.params.filename);
    const filePath = path.join("/var/uploads", safeName);
    if (!filePath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}
