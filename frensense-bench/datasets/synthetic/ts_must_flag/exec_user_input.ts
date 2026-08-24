import { exec } from "child_process";

export function runReport(req: any, res: any) {
  const cmd = req.body.command;
  exec(cmd, (err: any, stdout: any) => {
    res.send(stdout);
  });
}
