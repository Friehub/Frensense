// SAFE: Try-catch with spawn instead of exec
import { exec } from "child_process";
import { spawn } from "child_process";
const ALLOWED = new Set(["convert"]);

function convertImage(req: Request, res: Response) {
  try {
    if (!ALLOWED.has(req.body.filename)) return res.status(403).send("Not allowed");
    const child = spawn("convert", [req.body.filename, "-resize", "800x800", "output.jpg"]);
    child.stdout.on("data", data => res.json({ output: data.toString() }));
  } catch (err) { res.status(500).json({ error: err.message }); }
}
