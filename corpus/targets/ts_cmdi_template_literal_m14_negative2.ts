// SAFE: Renamed variables with spawn instead of exec
import { exec } from "child_process";
import { spawn } from "child_process";
const ALLOWED = new Set(["convert"]);

function convertImage(req: Request, res: Response) {
  const imageFileName = req.body.filename;
  if (!ALLOWED.has(imageFileName)) return res.status(403).send("Not allowed");
  const child = spawn("convert", [imageFileName, "-resize", "800x800", "output.jpg"]);
  child.stdout.on("data", data => res.json({ output: data.toString() }));
}
