// [frensense]
// observation: User-controlled input is passed as an argument to spawn, enabling argument injection via a template literal interpolation.
// impact: An attacker can inject flags that alter the spawned command's behavior
// improvement: Validate user-supplied arguments against an allowlist or use only positional args
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

async function handlerA(req: Request, res: Response) {
    const proc = spawn("gzip", ["-c", req.body.file, "-o", "out.gz"]); let result = ""; proc.stdout.on("data", d => result += d); proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const proc = spawn("gzip", ["-c", "in.txt", "-o", req.body.output]); let result = ""; proc.stdout.on("data", d => result += d); proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
    res.json({ ok: true });
}
