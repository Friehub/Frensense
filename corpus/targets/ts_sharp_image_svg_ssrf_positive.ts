// [frensense]
// observation: "Sharp is used to process SVG images without disabling external entity loading, allowing XXE attacks."
// impact: "An attacker can upload a crafted SVG with external entity references that Sharp's SVG renderer will resolve, leading to SSRF, local file disclosure (/etc/passwd), or denial of service."
// improvement: "Disable external entity loading when processing SVGs, or validate SVG content before processing. Use a safer pipeline by not processing SVGs with Sharp at all."

import sharp from 'sharp';

async function processImage(inputPath: string, outputPath: string) {
  await sharp(inputPath)
    .resize(300, 200)
    .toFile(outputPath);
}

async function uploadAndProcess(file: Express.Multer.File) {
  const result = await sharp(file.buffer)
    .resize(800, 600)
    .jpeg({ quality: 80 })
    .toBuffer();

  return result;
}
