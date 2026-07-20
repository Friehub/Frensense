// SAFE: Async path with allowlist and execFile
import { exec } from "child_process";
import { execFile } from "child_process";
const ALLOWED = new Set(["ls", "pwd", "date"]);

async function validateCmd(req: any): Promise<string> {
  const cmd = req.query.cmd;
  if (!ALLOWED.has(cmd)) throw new Error("Not allowed");
  return cmd;
}

async function handler(req: any, res: any) {
  try {
    const cmd = await validateCmd(req);
    execFile(cmd, [], (error, stdout) => {
      if (error) return res.status(500).send("Error");
      res.send(stdout);
    });
  } catch { res.status(403).send("Not allowed"); }
}
