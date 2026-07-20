// SAFE: Helper sanitizes filename before path join
import sanitize from "sanitize-filename";
import * as path from "path";

function resolveSafePath(base: string, userInput: string): string {
  const safe = sanitize(userInput);
  return path.join(base, safe);
}

function readFile(req: Request, res: Response) {
  const filePath = resolveSafePath("/var/uploads", req.params.filename);
  const content = fs.readFileSync(filePath, "utf-8");
  res.send(content);
}

function serveAsset(req: Request, res: Response) {
  const fullPath = resolveSafePath("/var/static", req.query.path);
  const data = fs.readFileSync(fullPath);
  res.type("application/octet-stream").send(data);
}
