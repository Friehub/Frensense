function handler(req: any, res: any) {
    const cmd = req.query.cmd;
    exec(cmd);
}
