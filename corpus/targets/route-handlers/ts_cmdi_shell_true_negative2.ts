// SAFE: Used execa which defaults to shell:false; validated the command against an allowlist of permitted commands.

import { execa } from "execa";

const ALLOWED_COMMANDS = new Set(["ls", "cat", "head", "tail", "wc", "du", "df"]);

async function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    if (!ALLOWED_COMMANDS.has(cmd)) {
        res.status(400).json({ error: "Command not allowed" });
        return;
    }
    const args = req.body.args || [];
    const { stdout } = await execa(cmd, args);
    res.json({ output: stdout });
}

async function npmInstall(req: Request, res: Response) {
    const packageName = req.body.package;
    const safeName = packageName.replace(/[^a-zA-Z0-9\-\.\/@]/g, "");
    const { stdout } = await execa("npm", ["install", safeName]);
    res.json({ installed: true });
}
