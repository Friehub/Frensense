// SAFE: Helper validates input, execFile prevents shell injection
const ALLOWED = new Set(["ls", "pwd", "date"]);

function runSafeCommand(cmd: string): Promise<string> {
  if (!ALLOWED.has(cmd)) return Promise.reject("Not allowed");
  return new Promise((resolve, reject) => {
    execFile(cmd, [], (error, stdout) => {
      if (error) reject(error);
      else resolve(stdout);
    });
  });
}

async function handler(req: any, res: any) {
  try {
    const output = await runSafeCommand(req.query.cmd);
    res.send(output);
  } catch {
    res.status(403).send("Not allowed");
  }
}
