// [frensense]
// observation: User-controlled input is passed directly as ffmpeg command arguments through a shell string, allowing arbitrary command execution via crafted filenames.
// impact: An attacker can inject shell metacharacters in filenames to execute arbitrary commands or use ffmpeg features like -f concat to read arbitrary files.
// improvement: Use spawn with arguments array and validate filenames against a strict allowlist pattern.

import { exec } from "child_process";

function transcodeVideo(req: Request, res: Response) {
    const inputFile = req.body.input;
    const outputFile = req.body.output;
    exec(`ffmpeg -i ${inputFile} -c:v libx264 -c:a aac ${outputFile}`, (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: stderr });
        res.json({ message: "Transcoded successfully" });
    });
}
