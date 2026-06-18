const { spawn } = require('child_process');
const express = require('express');
const app = express();

app.post('/ping', (req, res) => {
    const host = req.body.host;
    const proc = spawn('ping', ['-c', '3', host]);
    let output = '';
    proc.stdout.on('data', (data) => { output += data; });
    proc.on('close', () => { res.send(output); });
});
