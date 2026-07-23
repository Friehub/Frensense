// SAFE: Try-catch with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["convert", "git"];

function convertImage(req: Request, res: Response) {
    try { if (!ALLOWED.includes(req.body.filename)) return res.status(403).send("Not allowed"); exec(`convert ${req.body.filename} -resize 800x800 output.jpg`); } catch (err) { console.error(err); res.status(500).json({ error: err.message }); }
}

function gitClone(req: Request, res: Response) {
    try { if (!ALLOWED.includes(req.body.url)) return res.status(403).send("Not allowed"); exec(`git clone ${req.body.url} /repos/repo`); } catch (err) { console.error(err); res.status(500).json({ error: err.message }); }
}
