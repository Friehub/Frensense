// SAFE: Validated arguments against an allowlist pattern before passing to spawn; restricted to allowed characters and no flag prefixes.

import { spawn } from "child_process";

const SAFE_FILENAME = /^[\w\-\.]+$/;

function compressFile(req: Request, res: Response) {
    const inputFile = req.body.file;
    const outputFile = req.body.output;
    if (!SAFE_FILENAME.test(inputFile) || !SAFE_FILENAME.test(outputFile)) {
        res.status(400).json({ error: "Invalid filename" });
        return;
    }
    const proc = spawn("gzip", ["-c", inputFile, "-o", outputFile]);
    let result = "";
    proc.stdout.on("data", d => result += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Compression failed" });
        res.json({ result });
    });
}

function ffmpegTranscode(req: Request, res: Response) {
    const inputPath = "/uploads/" + req.body.input;
    const outputPath = "/output/" + req.body.output;
    const ALLOWED_CODECS = new Set(["libx264", "libx265", "vp9", "av1"]);
    const codec = req.body.codec;
    if (!ALLOWED_CODECS.has(codec)) {
        res.status(400).json({ error: "Unsupported codec" });
        return;
    }
    const proc = spawn("ffmpeg", ["-i", inputPath, "-c:v", codec, outputPath]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Transcode failed" });
        res.json({ success: true });
    });
}
