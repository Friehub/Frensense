// SAFE: Only a pre-defined set of tool names is allowed; user input maps to a command from the allowlist
import express from "express";
import { execFile } from "child_process";

const ALLOWED_TOOLS: Record<string, { path: string; allowedArgs: number }> = {
    "convert": { path: "/usr/bin/convert", allowedArgs: 2 },
    "identify": { path: "/usr/bin/identify", allowedArgs: 1 },
};

export function runTool(req: express.Request, res: express.Response) {
    const toolName = req.query.tool as string;
    const tool = ALLOWED_TOOLS[toolName];
    if (!tool) {
        return res.status(403).json({ error: "Tool not allowed" });
    }
    const args = (req.body.args as string[]) || [];
    if (args.length > tool.allowedArgs) {
        return res.status(400).json({ error: "Too many arguments" });
    }
    execFile(tool.path, args, { timeout: 10000 }, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
