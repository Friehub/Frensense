// SAFE: Template literal only used after allowlist check, execFile for safety
const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
  const cmd = req.query.cmd;
  if (!ALLOWED.includes(cmd)) return res.status(403).send("Not allowed");
  execFile("/usr/bin/" + cmd, [], (error, stdout) => {
    if (error) return res.status(500).send("Error");
    res.send(stdout);
  });
}
