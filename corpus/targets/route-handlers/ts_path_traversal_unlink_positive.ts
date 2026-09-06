// [frensense]
// observation: A user-controlled filename is passed to fs.unlink, fs.rm, or fs.rmdir without path validation, allowing deletion of arbitrary files outside the intended directory.
// impact: An attacker can delete critical system files, database files, or other users' data by providing a traversal path like "../../../etc/cron.d/cleanup".
// improvement: Validate and sanitize the path before deletion; restrict deletion to a specific directory.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import express from "express";
import fs from "fs";
import path from "path";

export function deleteFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = "/var/uploads/" + filename;
    fs.unlinkSync(filePath);
    res.json({ success: true });
}

export function removeDir(req: express.Request, res: express.Response) {
    const dir = req.query.dir as string;
    const dirPath = path.join("/var/temp", dir);
    fs.rmSync(dirPath, { recursive: true, force: true });
    res.json({ success: true });
}
