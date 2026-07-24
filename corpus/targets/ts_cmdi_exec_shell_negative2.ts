// SAFE: Command is selected from a fixed internal mapping; no user string
//       reaches the shell. execFile() used with static binary path.

import { execFile } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();

const JOB_MAP: Record<string, { bin: string; args: string[] }> = {
    report:  { bin: "/usr/local/bin/report-gen", args: ["--json"] },
    backup:  { bin: "/usr/local/bin/backup",     args: ["--incremental"] },
};

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const jobName = req.body.script as string;
    const job = JOB_MAP[jobName];
    if (!job) {
        return res.status(400).json({ error: "Unknown job" });
    }
    execFile(job.bin, job.args, (err, stdout) => {
        if (err) return res.status(500).json({ error: "Job failed" });
        res.json({ output: stdout });
    });
});

export default router;
