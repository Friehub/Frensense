// SAFE: Template literal not used for file path — uses sanitize-filename
import sanitize from "sanitize-filename";
import * as path from "path";

function downloadFile(req: Request, res: Response) {
  const name = sanitize(req.body.name);
  const file = fs.readFileSync(path.join("/data", name));
  res.send(file);
}

function readFile(req: Request, res: Response) {
  const filename = sanitize(req.params.filename);
  const content = fs.readFileSync(path.join("/var/uploads", filename), "utf-8");
  res.send(content);
}
