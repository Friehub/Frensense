// [frensense]
// observation: User-controlled input is interpolated into a shell command string inside a conditional block on the tainted branch.
// impact: An attacker can inject shell metacharacters.
// improvement: Use spawn without shell:true

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    if (req.body.filename) {
        exec(`convert ${req.body.filename} -resize 800x800 output.jpg`, (err, stdout) => {
            if (err) return res.status(500).json({ error: err.message });
            res.json({ output: stdout });
        });
    } else { res.status(400).send("Missing filename"); }
}

function gitClone(req: Request, res: Response) {
    if (req.body.url && req.body.url.length > 0) {
        exec(`git clone ${req.body.url} /repos/repo`, (err, stdout) => {
            if (err) return res.status(500).json({ error: err.message });
            res.json({ message: "Cloned successfully" });
        });
    }
}
