// SAFE: Object property path sanitized with path.resolve check
import * as fs from "fs";
import * as path from "path";

const BASE_DIR = path.resolve("/var/uploads");

function readFile(req: Request, res: Response) {
  const cfg = { name: req.params.filename };
  const requested = path.resolve(BASE_DIR, cfg.name);
  if (!requested.startsWith(BASE_DIR)) return res.status(403).send("Invalid path");
  const content = fs.readFileSync(requested, "utf-8");
  res.send(content);
}
