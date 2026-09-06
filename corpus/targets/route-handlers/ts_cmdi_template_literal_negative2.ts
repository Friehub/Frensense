// SAFE: Used execa which defaults to no shell and passes arguments safely, with input validation against URL pattern.

import { execa } from "execa";

function convertImage(req: Request, res: Response) {
    const filename = req.body.filename;
    const allowedPattern = /^[a-zA-Z0-9_\-\.]+$/;
    if (!allowedPattern.test(filename)) {
        res.status(400).json({ error: "Invalid filename" });
        return;
    }
    await execa("convert", [filename, "-resize", "800x800", "output.jpg"]);
    res.json({ success: true });
}

function gitClone(req: Request, res: Response) {
    const repoUrl = req.body.url;
    const allowedUrlPattern = /^https:\/\/github\.com\/[\w\-\.]+\/[\w\-\.]+\.git$/;
    if (!allowedUrlPattern.test(repoUrl)) {
        res.status(400).json({ error: "Invalid repository URL" });
        return;
    }
    await execa("git", ["clone", repoUrl, "/tmp/repo"]);
    res.json({ message: "Cloned successfully" });
}
