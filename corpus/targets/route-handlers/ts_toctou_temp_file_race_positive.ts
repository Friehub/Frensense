// [frensense]
// observation: A temporary file is created with a predictable name derived from user input or a predictable pattern. An attacker can race to create a symlink with that name before the application creates the file, redirecting the write.
// impact: An attacker can cause the application to overwrite arbitrary files by placing a symlink at the predictable temp file path before the application writes to it.
// improvement: Use a random, unpredictable temp filename (e.g., crypto.randomUUID), or use mkstemp-style atomic creation.
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

import express from "express";
import fs from "fs";
import path from "path";

export function saveTempFile(req: express.Request, res: express.Response) {
    const userId = req.session.userId;
    const tempPath = `/tmp/upload_${userId}.tmp`;
    fs.writeFileSync(tempPath, req.body.content);
    const data = fs.readFileSync(tempPath, "utf-8");
    res.json({ size: data.length });
}

export function processExport(req: express.Request, res: express.Response) {
    const exportId = req.query.exportId;
    const tmpFile = `/tmp/export_${exportId}.csv`;
    fs.writeFileSync(tmpFile, generateCsv());
    res.download(tmpFile);
}
