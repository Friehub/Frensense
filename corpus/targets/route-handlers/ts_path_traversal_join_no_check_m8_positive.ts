// [frensense]
// observation: User-controlled input is passed to path.join and used to read a file without prefix verification via an array element access.
// impact: An attacker can traverse directories to read arbitrary files on the server
// improvement: Verify the resolved path starts with the intended base directory
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

async function handlerA(req: Request, res: Response) {
    const arr = [req.params.filename];
    const filePath = path.join("/var/uploads", arr[0]); const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.query.path];
    const filePath = path.join("/var/static", items[0]); const content = fs.readFileSync(filePath); res.type("application/octet-stream").send(content);
    res.json({ ok: true });
}
