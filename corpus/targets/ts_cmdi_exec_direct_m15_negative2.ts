// SAFE: .then() chain with allowlist and execFile
import { exec } from "child_process";
import { execFile } from "child_process";
const ALLOWED = new Set(["ls", "pwd", "date"]);

function handler(req: any, res: any) {
  Promise.resolve(req.query.cmd).then(cmd => {
    if (!ALLOWED.has(cmd)) return res.status(403).send("Not allowed");
    execFile(cmd, [], (error, stdout) => { if (error) return res.status(500).send("Error"); res.send(stdout); });
  });
}
