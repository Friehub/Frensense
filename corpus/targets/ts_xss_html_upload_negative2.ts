// SAFE: File upload MIME type is validated server-side; only non-executable types are accepted
import express from "express";
import multer from "multer";
import path from "path";

const ALLOWED_MIME_TYPES = new Set(["image/png", "image/jpeg", "image/gif", "application/pdf"]);

const upload = multer({
    dest: "uploads/",
    fileFilter: (req, file, cb) => {
        if (!ALLOWED_MIME_TYPES.has(file.mimetype)) {
            cb(new Error("File type not allowed"));
            return;
        }
        cb(null, true);
    },
});

export function uploadHTML(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.setHeader("Content-Type", file.mimetype);
    res.setHeader("Content-Disposition", "attachment");
    res.sendFile(file.path);
}
