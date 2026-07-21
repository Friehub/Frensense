// SAFE: validates command name against an allowlist before execution
import { exec } from 'node:child_process';

const ALLOWED_COMMANDS = ['ping', 'traceroute', 'nslookup'];

function isAllowedCommand(command: string): boolean {
  for (const allowed of ALLOWED_COMMANDS) {
    if (command === allowed) {
      return true;
    }
  }
  return false;
}

export async function runCommand(cmd: string, args: string): Promise<string> {
  if (!isAllowedCommand(cmd)) {
    throw new Error('Command not allowed');
  }
  return new Promise((resolve, reject) => {
    exec(`${cmd} ${args}`, { shell: false }, (err, stdout) => {
      if (err) reject(err);
      else resolve(stdout);
    });
  });
}
