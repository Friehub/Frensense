// SAFE: Renamed variables with allowlist and execFile
import { exec } from "child_process";
import { execFile } from "child_process";
const ALLOWED = new Set(["ls", "pwd", "date"]);

function handleRequest(req: any, res: any) {
  const userCommand = req.query.cmd;
  if (!ALLOWED.has(userCommand)) return res.status(403).send("Not allowed");
  execFile(userCommand, [], (error, stdout) => { if (error) return res.status(500).send("Error"); res.send(stdout); });
}
