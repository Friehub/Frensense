function executeCommand(req: Request) {
    const cmd = req.body.command;
    exec(cmd);
}
