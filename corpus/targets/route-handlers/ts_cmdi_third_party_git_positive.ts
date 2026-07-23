// [frensense]
// observation: User-controlled URL is interpolated into a shell command passed to exec for git clone, allowing arbitrary command injection via the URL.
// impact: An attacker can inject shell metacharacters in the URL to execute arbitrary commands, or use options like --config to modify git configuration.
// improvement: Use spawn with separate args array and validate the URL against a strict allowlist pattern.

import { exec } from "child_process";

function cloneRepository(req: Request, res: Response) {
    const repoUrl = req.body.url;
    exec(`git clone ${repoUrl} /tmp/repo`, (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: stderr });
        res.json({ message: "Repository cloned" });
    });
}

function gitOperation(req: Request, res: Response) {
    const remote = req.body.remote;
    const branch = req.body.branch;
    exec(`git fetch ${remote} && git checkout ${branch}`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

function svnCheckout(req: Request, res: Response) {
    const svnUrl = req.body.url;
    exec(`svn checkout ${svnUrl} /tmp/svn-checkout`, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
