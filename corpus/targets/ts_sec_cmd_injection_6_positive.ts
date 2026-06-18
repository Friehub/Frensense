const { exec } = require('child_process');
const express = require('express');
const app = express();

app.get('/proxy', (req, res) => {
    const url = req.headers['x-target-url'];
    exec(`curl -s "${url}"`, (err, stdout) => {
        res.send(stdout);
    });
});
