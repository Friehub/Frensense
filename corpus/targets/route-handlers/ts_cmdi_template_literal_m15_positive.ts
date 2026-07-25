// [frensense]
// observation: User-controlled input is interpolated into a shell command string via a promise .then() chain.
// impact: An attacker can inject shell metacharacters.
// improvement: Use spawn without shell:true
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    Promise.resolve(req.body.filename).then(filename => {
        exec(`convert ${filename} -resize 800x800 output.jpg`);
    });
}

function gitClone(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.url)).then(url => {
        exec(`git clone ${url} /repos/repo`);
    });
}
