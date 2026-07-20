// SAFE: Uses fixed path prefix to prevent argument injection
import { spawn } from "child_process";
function handlerA(req: Request, res: Response) {
    const proc = spawn("gzip", ["-c", "/uploads/" + req.body.file, "-o", "out.gz"]);
    let result = ""; proc.stdout.on("data", d => result += d);
    proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
}
function handlerB(req: Request, res: Response) {
    const proc = spawn("cp", ["/input.txt", "/output/" + req.body.output]);
    proc.on("close", code => { if (code !== 0) return; res.json({ success: true }); });
}
