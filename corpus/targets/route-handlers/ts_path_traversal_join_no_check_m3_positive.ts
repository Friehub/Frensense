// [frensense]
// observation: User-controlled input is passed to path.join and used to read a file without prefix verification through multiple variable assignments.
// impact: An attacker can traverse directories to read arbitrary files on the server
// improvement: Verify the resolved path starts with the intended base directory

async function handlerA(req: Request, res: Response) {
    const a = req.params.filename;
    const b = a;
    const filePath = path.join("/var/uploads", b); const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.query.path;
    const y = x;
    const z = y;
    const filePath = path.join("/var/static", z); const content = fs.readFileSync(filePath); res.type("application/octet-stream").send(content);
    res.json({ ok: true });
}
