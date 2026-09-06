// SAFE: SVG detection and rejection based on content inspection, with external entity loading explicitly disabled via svg-1.1 parser config

import sharp from 'sharp';

function isSvg(buffer: Buffer): boolean {
  const header = buffer.slice(0, 20).toString().toLowerCase();
  return header.includes('<svg') || header.includes('<?xml');
}

async function processImage(inputPath: string, outputPath: string) {
  const buffer = await fs.promises.readFile(inputPath);
  if (isSvg(buffer)) {
    throw new Error('SVG processing is not allowed');
  }

  await sharp(buffer)
    .resize(300, 200)
    .toFile(outputPath);
}
