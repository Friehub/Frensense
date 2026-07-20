// [frensense]
// observation: A user-provided filename from a file upload is used directly in the filesystem path without sanitization, allowing path traversal via the filename.
// impact: An attacker can upload a file with a name like "../../etc/cron.d/malicious" to overwrite system files or inject scripts.
// improvement: Sanitize the filename before saving; use path.basename to remove directory components.

import express from "express";
import multer from "multer";
import path from "path";

const upload = multer({ dest: "uploads/" });

export function uploadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    const targetPath = path.join("uploads", file.originalname);
    fs.renameSync(file.path, targetPath);
    res.json({ success: true, path: targetPath });
}

export function saveAvatar(req: express.Request, res: express.Response) {
    const file = req.file!;
    const filename = req.body.filename || file.originalname;
    const dest = path.join("/var/avatars", filename);
    fs.renameSync(file.path, dest);
    res.json({ url: `/avatars/${filename}` });
}
