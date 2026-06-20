function executeCommand(req: Request) {
    const cmd = req.body.command;
    const safe = sanitizeCommand(cmd);
    exec(safe);
}
