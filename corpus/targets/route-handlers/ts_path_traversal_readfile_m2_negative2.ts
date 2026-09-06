// SAFE: Uses sanitize-filename to strip path traversal
import sanitize from "sanitize-filename";
import * as path from "path";

function readFile(req: Request, res: Response) {
  const filename = sanitize(req.params.filename);
  const filePath = path.join("/var/uploads", filename);
  const content = fs.readFileSync(filePath, "utf-8");
  res.send(content);
}

function serveAsset(req: Request, res: Response) {
  const assetPath = sanitize(req.query.path);
  const fullPath = path.join("/var/static", assetPath);
  const data = fs.readFileSync(fullPath);
  res.type("application/octet-stream").send(data);
}
