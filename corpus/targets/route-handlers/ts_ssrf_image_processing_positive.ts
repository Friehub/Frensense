// [frensense]
// observation: An image processing library (sharp, jimp) fetches an image from a user-provided URL, allowing SSRF via the image fetch.
// impact: The server makes HTTP requests to attacker-chosen URLs when processing images, including internal services or cloud metadata endpoints.
// improvement: Validate the image URL against an allowlist before fetching, or require image uploads instead of URL fetching.

import express from "express";
import sharp from "sharp";

export async function processImage(req: express.Request, res: express.Response) {
    const imageUrl = req.query.url as string;
    const response = await fetch(imageUrl);
    const buffer = Buffer.from(await response.arrayBuffer());
    const resized = await sharp(buffer).resize(200, 200).png().toBuffer();
    res.setHeader("Content-Type", "image/png");
    res.send(resized);
}
