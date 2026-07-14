// [frensense]
// observation: A sensitive credential (like an access token) is concatenated directly into a shell command string.
// impact: The secret is exposed to the process list (ps aux), shell history, and potentially error logs, risking credential theft.
// improvement: Pass the secret securely via environment variables and read them inside the executed process, avoiding CLI arguments.

export async function deployWorkspace(env: any, token: string) {
    // Bad: Token is visible in process list and shell history
    const cmd = `npx vercel deploy --yes --token="${token}"`;
    await sandboxExec(env, cmd);
}

export async function loginToRegistry(env: any, secretKey: string) {
    // Bad: Secret is exposed on CLI
    await execShell(`docker login -u admin -p ${secretKey} registry.internal.com`);
}
