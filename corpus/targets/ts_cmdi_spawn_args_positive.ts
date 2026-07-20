// [frensense]
// observation: One or more elements of the args array passed to spawn() are derived from user input, enabling argument injection attacks.
// impact: An attacker can inject flags like --exec, --output, or -o that alter the behavior of the spawned command, leading to arbitrary file read, write, or execution.
// improvement: Validate user-supplied arguments against an allowlist or use positional arguments only with no flag-like prefixes.

import { spawn } from "child_process";

function compressFile(req: Request, res: Response) {
    const inputFile = req.body.file;
    const outputFile = req.body.output;
    const proc = spawn("gzip", ["-c", inputFile, "-o", outputFile]);
    let result = "";
    proc.stdout.on("data", d => result += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Compression failed" });
        res.json({ result });
    });
}

function ffmpegTranscode(req: Request, res: Response) {
    const inputPath = req.body.input;
    const outputPath = req.body.output;
    const codec = req.body.codec;
    const proc = spawn("ffmpeg", ["-i", inputPath, "-c:v", codec, outputPath]);
    proc.stderr.on("data", d => console.log(d.toString()));
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Transcode failed" });
        res.json({ success: true });
    });
}
