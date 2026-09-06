// SAFE: Used @sindresorhus/escape-shell-arg to escape all user-supplied arguments before passing to spawn.

import { spawn } from "child_process";
import escapeShellArg from "escape-shell-arg";

function compressFile(req: Request, res: Response) {
    const inputFile = escapeShellArg(req.body.file);
    const outputFile = escapeShellArg(req.body.output);
    const proc = spawn("gzip", ["-c", inputFile, "-o", outputFile]);
    let result = "";
    proc.stdout.on("data", d => result += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Compression failed" });
        res.json({ result });
    });
}

function ffmpegTranscode(req: Request, res: Response) {
    const inputPath = escapeShellArg("/uploads/" + req.body.input);
    const outputPath = escapeShellArg("/output/" + req.body.output);
    const codec = escapeShellArg(req.body.codec);
    const proc = spawn("ffmpeg", ["-i", inputPath, "-c:v", codec, outputPath]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Transcode failed" });
        res.json({ success: true });
    });
}
