// [frensense]
// observation: A sensitive credential is provided to a shell process securely via environment variables.
// impact: None — environment variables are not globally exposed to the process list or shell history in the same way CLI arguments are.
// improvement: N/A — this is the correct pattern.

export async function deployWorkspace(env: any, token: string) {
    // Good: Token passed via env vars, not CLI args
    await sandboxExec(env, `npx vercel deploy --yes`, {
        env: { VERCEL_TOKEN: token }
    });
}

export async function loginToRegistry(env: any, secretKey: string) {
    // Good: Secret passed via stdin
    await execShell(`docker login --username admin --password-stdin registry.internal.com`, {
        input: secretKey
    });
}
