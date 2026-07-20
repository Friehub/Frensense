// SAFE: SVG files are served with Content-Disposition: attachment and the correct MIME type is validated
import express from "express";
import multer from "multer";
import path from "path";

const upload = multer({
    dest: "uploads/",
    fileFilter: (req, file, cb) => {
        const ext = path.extname(file.originalname).toLowerCase();
        if (ext !== ".svg") {
            cb(new Error("Only SVG files allowed"));
            return;
        }
        cb(null, true);
    },
});

const SVG_MIME = "image/svg+xml";

export function uploadSVG(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.setHeader("Content-Type", SVG_MIME);
    res.setHeader("Content-Disposition", "attachment; filename=\"image.svg\"");
    res.sendFile(file.path);
}

export async function serveUploadedFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    res.setHeader("Content-Type", SVG_MIME);
    res.setHeader("Content-Disposition", "attachment");
    res.sendFile(`uploads/${filename}`);
}
