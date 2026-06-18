const { exec } = require('child_process');
const express = require('express');
const app = express();

function buildCommand(params) {
    return `convert ${params.input} -resize ${params.width}x${params.height} ${params.output}`;
}

app.post('/resize', (req, res) => {
    const cmd = buildCommand(req.body);
    exec(cmd, (err, stdout) => {
        res.send(stdout || 'done');
    });
});
