// [frensense]
// observation: User-controlled input is interpolated into a shell command string passed to exec through multiple variable assignments.
// impact: An attacker can inject shell metacharacters to execute arbitrary OS commands
// improvement: Use spawn without shell:true and pass user input as separate arguments

async function handlerA(req: Request, res: Response) {
    const a = req.body.filename;
    const b = a;
    exec(`convert ${b} -resize 800x800 output.jpg`, (err, stdout) => { if (err) return; res.json({ output: stdout }); });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.body.url;
    const y = x;
    const z = y;
    exec(`git clone ${z} /tmp/repo`, (err, stdout) => { if (err) return; res.json({ message: "Cloned" }); });
    res.json({ ok: true });
}
