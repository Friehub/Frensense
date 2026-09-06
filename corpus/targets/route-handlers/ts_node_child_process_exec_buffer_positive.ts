// [frensense]
// observation: child_process.exec is called with the encoding option omitted, causing stdout to be returned as a Buffer that is then treated as a string, potentially leaking binary data.
// impact: Binary data or non-UTF8 output can corrupt string handling, leading to information disclosure or broken logic.
// improvement: Always specify an explicit encoding such as "utf8" when expecting text output from exec.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import { exec } from "node:child_process";

function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    exec(cmd, (error, stdout, stderr) => {
        if (error) return res.status(500).json({ error: stderr });
        res.send(stdout);
    });
}
