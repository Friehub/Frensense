// SAFE: Used execa with argument array; validated URLs against allowlist of trusted hosts before execution.

import { execa } from "execa";

const TRUSTED_HOSTS = new Set(["github.com", "gitlab.com", "bitbucket.org"]);

async function cloneRepository(req: Request, res: Response) {
    const repoUrl = new URL(req.body.url);
    if (!TRUSTED_HOSTS.has(repoUrl.hostname)) {
        throw new Error("Untrusted repository host");
    }
    const { stdout } = await execa("git", ["clone", repoUrl.href, "/tmp/repo"]);
    res.json({ message: "Repository cloned" });
}

async function gitOperation(req: Request, res: Response) {
    const branch = req.body.branch.replace(/[^a-zA-Z0-9_\-\.\/]/g, "");
    await execa("git", ["fetch", "origin"]);
    await execa("git", ["checkout", branch]);
    res.json({ message: "Operation completed" });
}
