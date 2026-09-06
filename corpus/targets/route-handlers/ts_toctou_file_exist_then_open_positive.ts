// [frensense]
// observation: The application checks if a file exists using fs.existsSync, then opens the same file path. Between the check and the open, an attacker can replace the file with a symlink to a different file (TOCTOU race).
// impact: An attacker can cause the application to read or write an unintended file by swapping the file between the existence check and the open call.
// improvement: Open the file directly and handle errors, rather than checking existence separately. Or use a single atomic operation.
// cwe: CWE-367
// cvss: 7.0
// owasp: 
// severity: High

import express from "express";
import fs from "fs";
import path from "path";

export function readUserFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/data", filename);
    if (!fs.existsSync(filePath)) {
        return res.status(404).send("File not found");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function deleteTempFile(req: express.Request, res: express.Response) {
    const filePath = req.body.path;
    if (fs.existsSync(filePath)) {
        fs.unlinkSync(filePath);
        res.json({ deleted: true });
    } else {
        res.json({ deleted: false });
    }
}
