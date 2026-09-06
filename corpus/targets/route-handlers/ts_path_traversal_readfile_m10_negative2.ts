// SAFE: Async path with path.resolve
import * as fs from "fs";
import * as path from "path";
const BASE_DIR = path.resolve("/var/uploads");

async function validatePath(req: any): Promise<string> {
  const name = req.params.filename;
  const resolved = path.resolve(BASE_DIR, name);
  if (!resolved.startsWith(BASE_DIR)) throw new Error("Invalid path");
  return resolved;
}

async function readFile(req: Request, res: Response) {
  try { const filePath = await validatePath(req); const content = fs.readFileSync(filePath, "utf-8"); res.send(content); } catch { res.status(403).send("Invalid path"); }
}
