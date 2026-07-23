// [frensense]
// observation: User-controlled input is interpolated into a file path via template literal before fs.readFileSync without sanitization.
// impact: An attacker can read arbitrary files by injecting path traversal sequences through template literal interpolation.
// improvement: Use path.basename() on user input before interpolation or use path.join with safe components.

import * as fs from "fs";
import * as path from "path";

function downloadFile(req: Request, res: Response) {
    const name = req.body.name;
    const file = fs.readFileSync(`/data/${name}`);
    res.send(file);
}

function readFile(req: Request, res: Response) {
    const content = fs.readFileSync(`/var/uploads/${req.params.filename}`, "utf-8");
    res.send(content);
}
