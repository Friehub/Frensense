// SAFE: .then() chain with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["convert", "git"];

function convertImage(req: Request, res: Response) {
    Promise.resolve(req.body.filename).then(filename => {
        if (!ALLOWED.includes(filename)) return res.status(403).send("Not allowed");
        exec(`convert ${filename} -resize 800x800 output.jpg`);
    });
}

function gitClone(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.url)).then(url => {
        if (!ALLOWED.includes(url)) return res.status(403).send("Not allowed");
        exec(`git clone ${url} /repos/repo`);
    });
}
