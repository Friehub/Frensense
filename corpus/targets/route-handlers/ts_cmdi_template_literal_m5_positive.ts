// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec via a template literal interpolation.
// impact: An attacker can inject shell metacharacters to execute arbitrary OS commands
// improvement: Use spawn without shell:true and pass user input as separate arguments

async function handlerA(req: Request, res: Response) {
    exec(`convert ${req.body.filename} -resize 800x800 output.jpg`, (err, stdout) => { if (err) return; res.json({ output: stdout }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    exec(`git clone ${req.body.url} /tmp/repo`, (err, stdout) => { if (err) return; res.json({ message: "Cloned" }); });
    res.json({ ok: true });
}
