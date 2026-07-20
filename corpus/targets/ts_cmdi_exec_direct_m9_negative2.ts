// SAFE: Object property validated with allowlist and execFile
import { exec } from "child_process";
import { execFile } from "child_process";

const ALLOWED = new Set(["ls", "pwd", "date"]);

function handler(req: any, res: any) {
  const cfg = { command: req.query.cmd };
  if (!ALLOWED.has(cfg.command)) return res.status(403).send("Not allowed");
  execFile(cfg.command, [], (error, stdout) => {
    if (error) return res.status(500).send("Error");
    res.send(stdout);
  });
}
