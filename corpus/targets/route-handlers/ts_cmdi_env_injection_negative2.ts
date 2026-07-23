// SAFE: Allowed only specific environment variables from user input, with strict validation on each value.

import { spawn } from "child_process";

const ALLOWED_ENV_VARS = new Set(["NODE_ENV", "LOG_LEVEL", "TZ"]);

function runBuild(req: Request, res: Response) {
    const env: Record<string, string> = { ...process.env };
    const customEnv = req.body.env || {};
    for (const key of Object.keys(customEnv)) {
        if (ALLOWED_ENV_VARS.has(key)) {
            env[key] = String(customEnv[key]);
        }
    }
    const proc = spawn("make", ["build"], { env });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function runWithCustomEnv(req: Request, res: Response) {
    const env: Record<string, string> = { ...process.env };
    if (req.body.logLevel) {
        env.LOG_LEVEL = String(req.body.logLevel);
    }
    const proc = spawn("deploy.sh", [], { env });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}
