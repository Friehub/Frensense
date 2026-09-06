// SAFE: .then() chain with spawn instead of exec
import { exec } from "child_process";
import { spawn } from "child_process";
const ALLOWED = new Set(["convert"]);

function convertImage(req: Request, res: Response) {
  Promise.resolve(req.body.filename).then(filename => {
    if (!ALLOWED.has(filename)) return res.status(403).send("Not allowed");
    const child = spawn("convert", [filename, "-resize", "800x800", "output.jpg"]);
    child.stdout.on("data", data => res.json({ output: data.toString() }));
  });
}
