// SAFE: Replaced exec with spawn, passing user input as separate arguments in an arguments array — no shell interpretation.

import { spawn, execSync } from "child_process";

function convertImage(req: Request, res: Response) {
    const filename = req.body.filename;
    const proc = spawn("convert", [filename, "-resize", "800x800", "output.jpg"]);
    let stdout = "";
    proc.stdout.on("data", d => stdout += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Conversion failed" });
        res.json({ output: stdout });
    });
}

function gitClone(req: Request, res: Response) {
    const repoUrl = req.body.url;
    const dest = req.body.destination;
    const proc = spawn("git", ["clone", repoUrl, `/repos/${dest}`]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Clone failed" });
        res.json({ message: "Cloned successfully" });
    });
}
