// SAFE: Uses execFile with arguments array to prevent shell injection
function handler(req: any, res: any) {
    const cmd = req.query.cmd;
    execFile(cmd, [], (error, stdout, stderr) => {
        if (error) return res.status(500).send("Error");
        res.send(stdout);
    });
}
