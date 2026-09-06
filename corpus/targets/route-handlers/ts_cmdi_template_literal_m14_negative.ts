// SAFE: Renamed variables with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["convert", "git"];

function convertImage(req: Request, res: Response) {
    const imageFileName = req.body.filename;
    if (!ALLOWED.includes(imageFileName)) return res.status(403).send("Not allowed");
    exec(`convert ${imageFileName} -resize 800x800 output.jpg`);
}

function gitClone(req: Request, res: Response) {
    const repositoryUrl = req.body.url;
    if (!ALLOWED.includes(repositoryUrl)) return res.status(403).send("Not allowed");
    exec(`git clone ${repositoryUrl} /repos/repo`);
}
