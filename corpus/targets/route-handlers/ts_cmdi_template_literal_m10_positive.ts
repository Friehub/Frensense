// [frensense]
// observation: User-controlled input is interpolated into a shell command string across an async/await boundary.
// impact: An attacker can inject shell metacharacters.
// improvement: Use spawn without shell:true
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

async function getFilename(req: any): Promise<string> { return req.body.filename; }
async function getRepoUrl(req: any): Promise<string> { return req.body.url; }

async function convertImage(req: Request, res: Response) {
    const filename = await getFilename(req);
    exec(`convert ${filename} -resize 800x800 output.jpg`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

async function gitClone(req: Request, res: Response) {
    const repoUrl = await getRepoUrl(req);
    exec(`git clone ${repoUrl} /repos/repo`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ message: "Cloned successfully" });
    });
}
