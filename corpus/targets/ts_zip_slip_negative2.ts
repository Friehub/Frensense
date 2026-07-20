// SAFE alternative: use path.relative to detect traversal
import AdmZip from 'adm-zip';
import path from 'path';
import fs from 'fs/promises';

async function extractSafe(zip: AdmZip, dest: string): Promise<void> {
  const base = path.resolve(dest);
  for (const entry of zip.getEntries()) {
    const fullPath = path.resolve(base, entry.entryName);
    const rel = path.relative(base, fullPath);
    if (rel.startsWith('..') || path.isAbsolute(rel)) {
      throw new Error('Zip slip detected');
    }
    if (!entry.isDirectory) {
      await fs.mkdir(path.dirname(fullPath), { recursive: true });
      await fs.writeFile(fullPath, entry.getData());
    }
  }
}
