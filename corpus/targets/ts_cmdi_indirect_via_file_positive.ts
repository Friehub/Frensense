// [frensense]
// observation: User-controlled input is written to a file that is later executed as a shell script or configuration, creating an indirect command injection path.
// impact: An attacker can inject shell commands into a file that gets executed, leading to arbitrary command execution when the script or config is processed.
// improvement: Avoid writing user input to executable files; if necessary, sanitize the input strictly before writing and avoid executing user-written files.

import { writeFileSync, chmodSync } from "fs";
import { exec } from "child_process";

function runUserScript(req: Request, res: Response) {
    const scriptContent = req.body.script;
    const scriptPath = "/tmp/user_script.sh";
    writeFileSync(scriptPath, scriptContent);
    chmodSync(scriptPath, "755");
    exec(`bash ${scriptPath}`, (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: stderr });
        res.json({ output: stdout });
    });
}

function writeConfigThenRun(req: Request, res: Response) {
    const configData = req.body.config;
    const configPath = "/tmp/app.config";
    writeFileSync(configPath, `[app]\ncommand = ${configData}`);
    exec(`/usr/bin/app --config ${configPath}`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
