// SAFE: Removed shell:true and passed command and arguments separately to spawn, preventing shell injection.

import { spawn } from "child_process";

function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    const args = req.body.args || [];
    const proc = spawn(cmd, args, { shell: false });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function runScript(req: Request, res: Response) {
    const scriptPath = "/scripts/" + req.body.script;
    const proc = spawn("/bin/bash", [scriptPath], { shell: false });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => {
        res.json({ code, output });
    });
}

function npmInstall(req: Request, res: Response) {
    const packageName = req.body.package;
    const proc = spawn("npm", ["install", packageName], { shell: false });
    proc.stderr.on("data", d => console.error(d.toString()));
    proc.on("close", code => {
        res.json({ installed: code === 0 });
    });
}
