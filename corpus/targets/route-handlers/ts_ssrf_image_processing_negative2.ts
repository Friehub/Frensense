// SAFE: Images are uploaded directly instead of fetched from URLs; no outbound request needed
import express from "express";
import multer from "multer";
import sharp from "sharp";

const upload = multer({ dest: "uploads/" });

export async function processImageUpload(req: express.Request, res: express.Response) {
    const file = req.file!;
    const resized = await sharp(file.path).resize(200, 200).png().toBuffer();
    res.setHeader("Content-Type", "image/png");
    res.send(resized);
}
