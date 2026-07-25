// [frensense]
// observation: The application checks file permissions or ownership, then accesses the file. Between the check and access, an attacker can swap the file (e.g., via a symlink rename), causing the application to operate on a different file.
// impact: An attacker can escalate privileges by making the application perform an operation on a file it should not have access to, after passing the permission check.
// improvement: Open the file first, then check permissions on the open file descriptor. Or use a single atomic check.
// cwe: CWE-367
// cvss: 7.0
// owasp: 
// severity: High

import express from "express";
import fs from "fs";
import path from "path";

export function readUserLog(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/logs/users", filename);
    const stat = fs.statSync(filePath);
    if (stat.uid !== req.session.userId) {
        return res.status(403).send("Not your log file");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function deleteFile(req: express.Request, res: express.Response) {
    const filePath = req.body.filePath;
    const stat = fs.statSync(filePath);
    if (!stat.isFile()) {
        return res.status(400).send("Not a file");
    }
    fs.unlinkSync(filePath);
    res.json({ success: true });
}
