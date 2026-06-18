const { exec } = require('child_process');
const express = require('express');
const app = express();

app.get('/search', (req, res) => {
    const query = req.query.q;
    const cmd = `grep -r "${query}" /data/`;
    exec(cmd, (err, stdout) => {
        res.send(stdout || 'no results');
    });
});
