const { exec } = require('child_process');
const express = require('express');
const app = express();

app.post('/deploy', (req, res) => {
    const env = req.body.environment;
    const ref = req.body.git_ref;
    const cmd = `deploy.sh --env ${env} --ref ${ref}`;
    exec(cmd, { cwd: '/app' }, (err, stdout) => {
        res.json({ output: stdout, error: err?.message });
    });
});
