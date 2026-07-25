// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec via string concatenation.
// impact: An attacker can inject shell metacharacters to execute arbitrary OS commands
// improvement: Use spawn without shell:true and pass user input as separate arguments
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

async function handlerA(req: Request, res: Response) {
    const cmd = "convert " + req.body.filename + " -resize 800x800 output.jpg"; exec(cmd, (err, stdout) => { if (err) return; res.json({ output: stdout }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const cmd = "git clone " + req.body.url + " /tmp/repo"; exec(cmd, (err, stdout) => { if (err) return; res.json({ message: "Cloned" }); });
    res.json({ ok: true });
}
