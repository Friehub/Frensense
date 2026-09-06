// SAFE: Replaced exec with spawn and argument array; validated archive filenames against a strict pattern.

import { spawn } from "child_process";

const SAFE_ARCHIVE = /^[\w\-\.]+\.(zip|tar\.gz|tgz|tar)$/;

function extractArchive(req: Request, res: Response) {
    const archive = req.body.archive;
    const destDir = "/extract/" + req.body.destination;
    if (!SAFE_ARCHIVE.test(archive)) {
        res.status(400).json({ error: "Invalid archive name" });
        return;
    }
    const proc = spawn("unzip", ["-o", archive, "-d", destDir]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Extraction failed" });
        res.json({ message: "Extracted successfully" });
    });
}

function compressFiles(req: Request, res: Response) {
    const output = req.body.output;
    if (!SAFE_ARCHIVE.test(output)) {
        res.status(400).json({ error: "Invalid output name" });
        return;
    }
    const files = req.body.files.filter((f: string) => /^[\w\-\.\/]+$/.test(f));
    const proc = spawn("zip", ["-r", output, ...files]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Compression failed" });
        res.json({ message: "Compressed successfully" });
    });
}

function untarArchive(req: Request, res: Response) {
    const archive = req.query.file as string;
    if (!SAFE_ARCHIVE.test(archive)) {
        res.status(400).json({ error: "Invalid archive name" });
        return;
    }
    const proc = spawn("tar", ["-xzf", archive, "-C", "/tmp/extract"]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Extraction failed" });
        res.json({ message: "Extracted successfully" });
    });
}
