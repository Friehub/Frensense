// [frensense]
// observation: User-controlled input is concatenated into a file path string before fs.readFileSync without sanitization.
// impact: An attacker can read arbitrary files by injecting path traversal sequences through string concatenation.
// improvement: Use path.basename() on user input and path.join() for safe path construction.

import * as fs from "fs";

function downloadFile(req: Request, res: Response) {
    const name = req.body.name;
    const file = fs.readFileSync("/data/" + name);
    res.send(file);
}

function readFile(req: Request, res: Response) {
    const content = fs.readFileSync("/var/uploads/" + req.params.filename, "utf-8");
    res.send(content);
}
