// SAFE: SVG files are rejected before reaching Sharp's pipeline

import sharp from 'sharp';

const ALLOWED_MIME_TYPES = new Set(['image/jpeg', 'image/png', 'image/webp']);

async function processImage(inputPath: string, outputPath: string, mimeType: string) {
  if (!ALLOWED_MIME_TYPES.has(mimeType)) {
    throw new Error('Unsupported image type');
  }

  await sharp(inputPath)
    .resize(300, 200)
    .toFile(outputPath);
}

async function uploadAndProcess(file: Express.Multer.File) {
  if (!ALLOWED_MIME_TYPES.has(file.mimetype)) {
    throw new Error('Only JPEG, PNG, and WebP are allowed');
  }

  const result = await sharp(file.buffer)
    .resize(800, 600)
    .jpeg({ quality: 80 })
    .toBuffer();

  return result;
}
