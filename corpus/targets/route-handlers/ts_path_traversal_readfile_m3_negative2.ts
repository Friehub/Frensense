// SAFE: Multi-hop with sanitize-filename
import sanitize from "sanitize-filename";
import * as path from "path";

function readFile(req: Request, res: Response) {
  const a = req.params.filename;
  const b = a;
  const filename = sanitize(b);
  const filePath = path.join("/var/uploads", filename);
  const content = fs.readFileSync(filePath, "utf-8");
  res.send(content);
}

function serveAsset(req: Request, res: Response) {
  const raw = req.query.path;
  const assetPath = sanitize(raw);
  const fullPath = path.join("/var/static", assetPath);
  const data = fs.readFileSync(fullPath);
  res.type("application/octet-stream").send(data);
}
