// SAFE: Conditional branch with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["convert", "git"];

function convertImage(req: Request, res: Response) {
    if (req.body.filename) {
        if (!ALLOWED.includes(req.body.filename)) return res.status(403).send("Not allowed");
        exec(`convert ${req.body.filename} -resize 800x800 output.jpg`);
    } else { res.status(400).send("Missing filename"); }
}

function gitClone(req: Request, res: Response) {
    if (req.body.url && req.body.url.length > 0) {
        if (!ALLOWED.includes(req.body.url)) return res.status(403).send("Not allowed");
        exec(`git clone ${req.body.url} /repos/repo`);
    }
}
