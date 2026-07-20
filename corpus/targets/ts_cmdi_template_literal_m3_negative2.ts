// SAFE: Uses execFile with arguments array and allowlist
import { execFile } from "child_process";
function handlerA(req: Request, res: Response) {
    const allowed = /^[\w\-\.]+$/;
    const filename = req.body.filename;
    if (!allowed.test(filename)) return res.status(403).send("Invalid");
    execFile("convert", [filename, "-resize", "800x800", "output.jpg"], (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
function handlerB(req: Request, res: Response) {
    execFile("git", ["clone", req.body.url, "/tmp/repo"], (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ message: "Cloned" });
    });
}
