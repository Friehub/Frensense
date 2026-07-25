// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec through a helper function.
// impact: An attacker can inject shell metacharacters to execute arbitrary OS commands
// improvement: Use spawn without shell:true and pass user input as separate arguments
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.body.filename);
    exec(`convert ${val} -resize 800x800 output.jpg`, (err, stdout) => { if (err) return; res.json({ output: stdout }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.body.url);
    exec(`git clone ${val} /tmp/repo`, (err, stdout) => { if (err) return; res.json({ message: "Cloned" }); });
    res.json({ ok: true });
}
