// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec, allowing arbitrary command execution through metacharacters through an object property.
// impact: An attacker can inject shell metacharacters (;, `, $(), ||) to execute arbitrary OS commands.
// improvement: Use spawn without shell:true and pass user input as separate arguments
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    const cfg = { filename: req.body.filename };
    exec(`convert ${cfg.filename} -resize 800x800 output.jpg`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

function gitClone(req: Request, res: Response) {
    const opts = { url: req.body.url };
    exec(`git clone ${opts.url} /repos/repo`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ message: "Cloned successfully" });
    });
}
