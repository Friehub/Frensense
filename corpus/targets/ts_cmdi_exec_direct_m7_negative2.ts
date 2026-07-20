// SAFE: Destructured input validated, execFile prevents shell injection
const ALLOWED = new Set(["ls", "pwd", "date"]);

function handler(req: any, res: any) {
  const { cmd } = req.query;
  if (!ALLOWED.has(cmd)) return res.status(403).send("Not allowed");
  execFile(cmd, [], (error, stdout) => {
    if (error) return res.status(500).send("Error");
    res.send(stdout);
  });
}
