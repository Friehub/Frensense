// SAFE: The tool name is sanitized and the path is resolved against a restricted directory, preventing path traversal
import express from "express";
import { execFile } from "child_process";
import path from "path";

const TOOLS_DIR = path.resolve("/usr/local/bin");

export function runTool(req: express.Request, res: express.Response) {
    const tool = req.query.tool as string;
    const safeName = path.basename(tool).replace(/[^a-zA-Z0-9_-]/g, "");
    const toolPath = path.join(TOOLS_DIR, safeName);
    if (!toolPath.startsWith(TOOLS_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    const args = req.body.args as string[];
    execFile(toolPath, args, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

export function convertFile(req: express.Request, res: express.Response) {
    return res.status(403).json({ error: "Custom converter not supported" });
}
