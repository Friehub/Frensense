// SAFE: Replaced exec with spawn and validated URL against a strict pattern before passing as argument.

import { spawn } from "child_process";

const ALLOWED_GIT_URL = /^https:\/\/github\.com\/[\w\-\.]+\/[\w\-\.]+(\.git)?$/;

function cloneRepository(req: Request, res: Response) {
    const repoUrl = req.body.url;
    if (!ALLOWED_GIT_URL.test(repoUrl)) {
        res.status(400).json({ error: "Invalid repository URL" });
        return;
    }
    const proc = spawn("git", ["clone", repoUrl, "/tmp/repo"]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Clone failed" });
        res.json({ message: "Repository cloned" });
    });
}

function gitOperation(req: Request, res: Response) {
    const proc = spawn("git", ["fetch", "origin"]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Fetch failed" });
        const branch = req.body.branch.replace(/[^a-zA-Z0-9_\-\.\/]/g, "");
        const checkout = spawn("git", ["checkout", branch]);
        checkout.on("close", c2 => {
            if (c2 !== 0) return res.status(500).json({ error: "Checkout failed" });
            res.json({ message: "Operation completed" });
        });
    });
}

function svnCheckout(req: Request, res: Response) {
    const svnUrl = req.body.url;
    if (!svnUrl.startsWith("https://")) {
        res.status(400).json({ error: "Only HTTPS URLs allowed" });
        return;
    }
    const proc = spawn("svn", ["checkout", svnUrl, "/tmp/svn-checkout"]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Checkout failed" });
        res.json({ message: "Checked out" });
    });
}
