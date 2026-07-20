// SAFE: Image URL is validated against an allowlist before fetching
import express from "express";
import sharp from "sharp";

const ALLOWED_DOMAINS = new Set(["cdn.example.com", "images.example.com"]);

function isAllowed(url: string): boolean {
    try {
        const parsed = new URL(url);
        return parsed.protocol.startsWith("http") && ALLOWED_DOMAINS.has(parsed.hostname);
    } catch {
        return false;
    }
}

export async function processImage(req: express.Request, res: express.Response) {
    const imageUrl = req.query.url as string;
    if (!isAllowed(imageUrl)) {
        return res.status(403).json({ error: "Image URL not allowed" });
    }
    const response = await fetch(imageUrl, { signal: AbortSignal.timeout(10000) });
    const buffer = Buffer.from(await response.arrayBuffer());
    const resized = await sharp(buffer).resize(200, 200).png().toBuffer();
    res.setHeader("Content-Type", "image/png");
    res.send(resized);
}
