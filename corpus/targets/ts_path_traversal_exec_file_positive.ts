// [frensense]
// observation: A user-controlled filename is concatenated with a base path and passed to child_process.execFile, allowing execution of arbitrary binaries via path traversal.
// impact: An attacker can execute arbitrary executables by providing a path like "../../usr/bin/wget" or "../../tmp/malicious.sh", bypassing intended restrictions.
// improvement: Validate and sanitize the path before execution; restrict execution to a specific directory or use a command allowlist.

import express from "express";
import { execFile } from "child_process";

export function runTool(req: express.Request, res: express.Response) {
    const tool = req.query.tool as string;
    const args = req.body.args as string[];
    execFile("/usr/local/bin/" + tool, args, (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}

export function convertFile(req: express.Request, res: express.Response) {
    const converter = req.body.converter;
    const input = req.body.input;
    execFile(converter, [input], (err, stdout) => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ output: stdout });
    });
}
