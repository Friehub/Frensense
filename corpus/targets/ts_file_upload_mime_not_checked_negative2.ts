// SAFE alternative: validate first 8 bytes (magic numbers)
const MAGIC_BYTES: Record<string, Uint8Array> = {
  'image/jpeg': new Uint8Array([0xFF, 0xD8, 0xFF]),
  'image/png': new Uint8Array([0x89, 0x50, 0x4E, 0x47]),
  'image/webp': new Uint8Array([0x52, 0x49, 0x46, 0x46]),
};

async function validateMagicBytes(filePath: string, expectedMime: string): Promise<boolean> {
  const handle = await fs.open(filePath, 'r');
  const buf = Buffer.alloc(8);
  await handle.read(buf, 0, 8, 0);
  await handle.close();
  const magic = MAGIC_BYTES[expectedMime];
  return magic && buf.slice(0, magic.length).equals(magic);
}
