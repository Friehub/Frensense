// SAFE: Writes secret to a temporary file and passes the file path instead of the value
import fs from "fs/promises";
import path from "path";
import os from "os";

export async function deployWorkspace(env: any, token: string) {
  const tmpFile = path.join(os.tmpdir(), `token_${Date.now()}`);
  await fs.writeFile(tmpFile, token, { mode: 0o600 });
  await sandboxExec(env, `npx vercel deploy --yes --token-file="${tmpFile}"`);
  await fs.unlink(tmpFile);
}

export async function loginToRegistry(env: any, secretKey: string) {
  const tmpFile = path.join(os.tmpdir(), `secret_${Date.now()}`);
  await fs.writeFile(tmpFile, secretKey, { mode: 0o600 });
  await execShell(`docker login -u admin --password-file "${tmpFile}" registry.internal.com`);
  await fs.unlink(tmpFile);
}
