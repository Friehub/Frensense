// [frensense]
// observation: User-controlled input is interpolated into a shell command string inside a try-catch block.
// impact: An attacker can inject shell metacharacters, with errors silently caught.
// improvement: Use spawn without shell:true
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    try { exec(`convert ${req.body.filename} -resize 800x800 output.jpg`); } catch (err) { console.error(err); }
}

function gitClone(req: Request, res: Response) {
    try { exec(`git clone ${req.body.url} /repos/repo`); } catch {}
}
