// SAFE: Only the basename of the uploaded filename is used; directory separators are stripped
import express from "express";
import multer from "multer";
import path from "path";
import fs from "fs";

const AVATAR_DIR = path.resolve("/var/avatars");

const upload = multer({ dest: "uploads/" });

export function uploadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    const safeName = path.basename(file.originalname).replace(/[^a-zA-Z0-9._-]/g, "_");
    const targetPath = path.join("uploads", safeName);
    fs.renameSync(file.path, targetPath);
    res.json({ success: true, path: targetPath });
}

export function saveAvatar(req: express.Request, res: express.Response) {
    const file = req.file!;
    const safeName = path.basename(req.body.filename || file.originalname).replace(/[^a-zA-Z0-9._-]/g, "_");
    const dest = path.resolve(AVATAR_DIR, safeName);
    if (!dest.startsWith(AVATAR_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    fs.renameSync(file.path, dest);
    res.json({ url: `/avatars/${safeName}` });
}
