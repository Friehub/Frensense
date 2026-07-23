// [frensense]
// observation: spawn is called with shell:true and a user-controlled command string, allowing arbitrary shell command execution through metacharacters.
// impact: An attacker can inject shell metacharacters (;, &&, |, $()) to execute arbitrary commands on the server, leading to full system compromise.
// improvement: Set shell to false (or omit it) and pass the command and arguments separately; avoid shell:true with any user-controlled input.

import { spawn } from "child_process";

function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    const proc = spawn(cmd, { shell: true });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function runScript(req: Request, res: Response) {
    const userScript = req.body.script;
    const proc = spawn(userScript, { shell: "/bin/bash" });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function npmInstall(req: Request, res: Response) {
    const packageName = req.body.package;
    const proc = spawn("npm install " + packageName, { shell: true });
    proc.stderr.on("data", d => console.error(d.toString()));
    proc.on("close", code => {
        res.json({ installed: code === 0 });
    });
}
