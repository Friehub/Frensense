// SAFE: validates file path against an allowed directory prefix
import { readFile } from 'node:fs/promises';
import { join, normalize } from 'node:path';

const ALLOWED_DIRS = ['/var/data/uploads', '/var/data/exports'];

function isPathSafe(filePath: string): boolean {
  const resolved = normalize(filePath);
  for (const dir of ALLOWED_DIRS) {
    if (resolved.startsWith(dir)) {
      return true;
    }
  }
  return false;
}

export async function loadFile(filePath: string): Promise<string> {
  if (!isPathSafe(filePath)) {
    throw new Error('Access denied');
  }
  return await readFile(filePath, 'utf-8');
}
