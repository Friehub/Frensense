// [frensense]
// observation: User-controlled filenames and paths are interpolated into shell commands for unzip/tar, enabling command injection through crafted archive or path names.
// impact: An attacker can inject shell metacharacters in filenames or paths to execute arbitrary commands, or use path traversal in archive contents (zip slip).
// improvement: Use spawn with separate args and validate all user-supplied paths against an allowlist.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

function extractArchive(req: Request, res: Response) {
    const archive = req.body.archive;
    const destDir = req.body.destination;
    exec(`unzip ${archive} -d ${destDir}`, (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: stderr });
        res.json({ message: "Extracted successfully" });
    });
}

function compressFiles(req: Request, res: Response) {
    const files = req.body.files.join(" ");
    const output = req.body.output;
    exec(`zip -r ${output} ${files}`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

function untarArchive(req: Request, res: Response) {
    const archive = req.query.file as string;
    exec(`tar -xzf ${archive} -C /tmp/extract`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
