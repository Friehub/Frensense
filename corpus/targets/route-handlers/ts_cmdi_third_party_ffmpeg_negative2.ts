// SAFE: Used spawn with arguments array and validated filenames against a strict pattern.

import { spawn } from "child_process";

const SAFE_NAME = /^[\w\-\.]+\.(mp4|avi|mkv|mov|webm)$/;

function transcodeVideo(req: Request, res: Response) {
    const inputFile = "/uploads/" + req.body.input;
    const outputFile = "/output/" + req.body.output;
    if (!SAFE_NAME.test(req.body.input) || !SAFE_NAME.test(req.body.output)) {
        res.status(400).json({ error: "Invalid filename" });
        return;
    }
    const proc = spawn("ffmpeg", ["-i", inputFile, "-c:v", "libx264", "-c:a", "aac", outputFile]);
    proc.on("close", code => {
        if (code !== 0) {
            res.status(500).json({ error: "Transcode failed" });
            return;
        }
        res.json({ message: "Transcoded successfully" });
    });
}
