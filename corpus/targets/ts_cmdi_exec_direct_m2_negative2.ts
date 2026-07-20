// SAFE: Uses execFile with arguments array and allowlist
function handler(req: any, res: any) {
    const allowed = ["ls", "pwd", "date"];
    const cmd = req.query.cmd;
    if (allowed.includes(cmd)) {
        execFile(cmd, [], (error, stdout) => {
            if (error) return res.status(500).send("Error");
            res.send(stdout);
        });
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    execFile("safe-task", [req.body.task], (error, stdout) => {
        if (error) return res.status(500).send("Error");
        res.send(stdout);
    });
}
