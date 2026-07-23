// SAFE: Array element validated, execFile prevents shell injection
const ALLOWED = new Set(["ls", "pwd", "date"]);

function handler(req: any, res: any) {
  const cmds = [req.query.cmd];
  if (!ALLOWED.has(cmds[0])) return res.status(403).send("Not allowed");
  execFile(cmds[0], [], (error, stdout) => {
    if (error) return res.status(500).send("Error");
    res.send(stdout);
  });
}
