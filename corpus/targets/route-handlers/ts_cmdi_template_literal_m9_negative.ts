// SAFE: Object property validated before exec
import { exec } from "child_process";

const ALLOWED = ["convert", "git"];

function convertImage(req: Request, res: Response) {
    const cfg = { filename: req.body.filename };
    if (!ALLOWED.includes(cfg.filename)) return res.status(403).send("Not allowed");
    exec(`convert ${cfg.filename} -resize 800x800 output.jpg`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

function gitClone(req: Request, res: Response) {
    const opts = { url: req.body.url };
    if (!ALLOWED.includes(opts.url)) return res.status(403).send("Not allowed");
    exec(`git clone ${opts.url} /repos/repo`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ message: "Cloned successfully" });
    });
}
