// SAFE: Object property validated, spawn used instead of exec
import { exec } from "child_process";
import { spawn } from "child_process";

const ALLOWED = new Set(["convert", "git"]);

function convertImage(req: Request, res: Response) {
  const cfg = { filename: req.body.filename };
  if (!ALLOWED.has(cfg.filename)) return res.status(403).send("Not allowed");
  const child = spawn("convert", [cfg.filename, "-resize", "800x800", "output.jpg"]);
  child.stdout.on("data", data => res.json({ output: data.toString() }));
}
