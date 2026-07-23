// SAFE: Async path with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["convert", "git"];

async function getFilename(req: any): Promise<string> {
    const name = req.body.filename; if (!ALLOWED.includes(name)) throw new Error("Not allowed"); return name;
}

async function getRepoUrl(req: any): Promise<string> {
    const url = req.body.url; if (!ALLOWED.includes(url)) throw new Error("Not allowed"); return url;
}

async function convertImage(req: Request, res: Response) {
    try { const filename = await getFilename(req); exec(`convert ${filename} -resize 800x800 output.jpg`); } catch { res.status(403).send("Not allowed"); }
}

async function gitClone(req: Request, res: Response) {
    try { const repoUrl = await getRepoUrl(req); exec(`git clone ${repoUrl} /repos/repo`); } catch { res.status(403).send("Not allowed"); }
}
