// [frensense]
// observation: User-controlled input is interpolated into a shell command string with renamed variables.
// impact: An attacker can inject shell metacharacters.
// improvement: Use spawn without shell:true

import { exec } from "child_process";

function convertImage(req: Request, res: Response) {
    const imageFileName = req.body.filename;
    exec(`convert ${imageFileName} -resize 800x800 output.jpg`);
}

function gitClone(req: Request, res: Response) {
    const repositoryUrl = req.body.url;
    exec(`git clone ${repositoryUrl} /repos/repo`);
}
