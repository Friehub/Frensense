// SAFE: Implements safe alternative
// SAFE: Validated arguments against an allowlist pattern
import { spawn } from "child_process";
const SAFE = /^[\w\-\.]+$/;
function handlerA(req: Request, res: Response) {
    const inputFile = req.body.file;
    if (!SAFE.test(inputFile)) return res.status(400).json({ error: "Invalid" });
    const proc = spawn("gzip", ["-c", inputFile, "-o", "out.gz"]);
    let result = ""; proc.stdout.on("data", d => result += d);
    proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
}
function handlerB(req: Request, res: Response) {
    const outputFile = req.body.output;
    if (!SAFE.test(outputFile)) return res.status(400).json({ error: "Invalid" });
    const proc = spawn("gzip", ["-c", "in.txt", "-o", outputFile]);
    let result = ""; proc.stdout.on("data", d => result += d);
    proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
}
