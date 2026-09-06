// SAFE: Async path with spawn instead of exec
import { exec } from "child_process";
import { spawn } from "child_process";
const ALLOWED = new Set(["convert"]);

async function validateAndConvert(req: any): Promise<string> {
  const name = req.body.filename; if (!ALLOWED.has(name)) throw new Error("Not allowed"); return name;
}

async function convertImage(req: Request, res: Response) {
  try { const filename = await validateAndConvert(req); const child = spawn("convert", [filename, "-resize", "800x800", "output.jpg"]); child.stdout.on("data", data => res.json({ output: data.toString() })); } catch { res.status(403).send("Not allowed"); }
}
