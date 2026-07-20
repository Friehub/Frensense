// [frensense]
// observation: User-controlled input is passed as an argument to spawn, enabling argument injection via destructured object property.
// impact: An attacker can inject flags that alter the spawned command's behavior
// improvement: Validate user-supplied arguments against an allowlist or use only positional args

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const proc = spawn("gzip", ["-c", input, "-o", "out.gz"]); let result = ""; proc.stdout.on("data", d => result += d); proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const proc = spawn("gzip", ["-c", "in.txt", "-o", value]); let result = ""; proc.stdout.on("data", d => result += d); proc.on("close", code => { if (code !== 0) return; res.json({ result }); });
    res.json({ ok: true });
}
