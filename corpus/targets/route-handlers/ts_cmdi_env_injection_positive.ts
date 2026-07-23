// [frensense]
// observation: User-controlled environment variables are passed to child_process spawn options, allowing injection of PATH, LD_PRELOAD, or other malicious environment variables.
// impact: An attacker can manipulate the dynamic linker via LD_PRELOAD to load a malicious shared library, or override PATH to execute a different binary than intended.
// improvement: Never pass user-controlled environment variables directly; use only a fixed set of allowed environment variables with validated values.

import { spawn } from "child_process";

function runBuild(req: Request, res: Response) {
    const customPath = req.body.customPath;
    const proc = spawn("make", ["build"], {
        env: {
            ...process.env,
            PATH: customPath,
        },
    });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function runWithCustomEnv(req: Request, res: Response) {
    const envVars = req.body.env;
    const proc = spawn("deploy.sh", [], {
        env: { ...process.env, ...envVars },
    });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}
