function handler(req: any, res: any) {
    const allowed = ["ls", "pwd", "date"];
    const cmd = req.query.cmd;
    if (allowed.includes(cmd)) {
        exec(cmd);
    } else {
        res.status(403).send("Command not allowed");
    }
}
