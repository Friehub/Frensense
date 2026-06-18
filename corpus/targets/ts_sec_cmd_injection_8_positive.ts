const { exec } = require('child_process');
const express = require('express');
const app = express();

app.get('/backup', (req, res) => {
    const filename = req.query.file;
    const cmd = 'tar czf /tmp/backup.tar.gz ' + filename;
    exec(cmd, (err) => {
        res.send(err ? 'error' : 'backup created');
    });
});
