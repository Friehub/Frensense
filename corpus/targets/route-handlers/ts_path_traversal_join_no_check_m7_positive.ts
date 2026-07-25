// [frensense]
// observation: User-controlled input is passed to path.join and used to read a file without prefix verification via destructured object property.
// impact: An attacker can traverse directories to read arbitrary files on the server
// improvement: Verify the resolved path starts with the intended base directory
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const filePath = path.join("/var/uploads", input); const content = fs.readFileSync(filePath, "utf-8"); res.send(content);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const filePath = path.join("/var/static", value); const content = fs.readFileSync(filePath); res.type("application/octet-stream").send(content);
    res.json({ ok: true });
}
