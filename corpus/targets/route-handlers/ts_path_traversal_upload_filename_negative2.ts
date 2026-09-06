// SAFE: Uses a server-generated UUID filename; original user filename is discarded
import express from "express";
import multer from "multer";
import path from "path";
import fs from "fs";
import crypto from "crypto";

const AVATAR_DIR = path.resolve("/var/avatars");

const storage = multer.diskStorage({
    destination: (req, file, cb) => cb(null, AVATAR_DIR),
    filename: (req, file, cb) => {
        const ext = path.extname(file.originalname);
        cb(null, crypto.randomUUID() + ext);
    },
});

const upload = multer({ storage });

export function uploadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.json({ success: true, filename: file.filename });
}

export function saveAvatar(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.json({ url: `/avatars/${file.filename}` });
}
