// SAFE: Stopped passing user-controlled env vars to spawn; only inherit process.env without modifications.

import { spawn } from "child_process";

function runBuild(req: Request, res: Response) {
    const proc = spawn("make", ["build"], {
        env: process.env,
    });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function runWithCustomEnv(req: Request, res: Response) {
    const proc = spawn("deploy.sh", [], {
        env: process.env,
    });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}
