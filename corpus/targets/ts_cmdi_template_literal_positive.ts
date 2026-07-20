// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec/spawn with shell:true, allowing arbitrary command execution through metacharacters.
// impact: An attacker can inject shell metacharacters (;, `, $(), ||) to execute arbitrary OS commands on the server, leading to full compromise.
// improvement: Use spawn without shell:true and pass user input as separate arguments, or validate/escape the input against an allowlist.

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    const filename = req.body.filename;
    exec(`convert ${filename} -resize 800x800 output.jpg`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

function gitClone(req: Request, res: Response) {
    const repoUrl = req.body.url;
    const dest = req.body.destination;
    exec(`git clone ${repoUrl} /repos/${dest}`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ message: "Cloned successfully" });
    });
}

function gitCloneSync(req: Request, res: Response) {
    const repoUrl = req.body.url;
    const result = execSync(`git clone ${repoUrl} /tmp/repo`).toString();
    res.json({ result });
}
