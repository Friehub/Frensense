// SAFE: Implements safe alternative
// SAFE: Uses spawn with arguments array — no shell interpretation
import { spawn } from "child_process";
function handlerA(req: Request, res: Response) {
    const filename = req.body.filename;
    const proc = spawn("convert", [filename, "-resize", "800x800", "output.jpg"]);
    let stdout = "";
    proc.stdout.on("data", d => stdout += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Failed" });
        res.json({ output: stdout });
    });
}
function handlerB(req: Request, res: Response) {
    const url = req.body.url;
    const proc = spawn("git", ["clone", url, "/tmp/repo"]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Failed" });
        res.json({ message: "Cloned" });
    });
}
