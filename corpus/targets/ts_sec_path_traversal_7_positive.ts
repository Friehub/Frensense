const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.get('/log', (req, res) => {
    const logFile = req.query.file;
    const logDir = '/var/log/app';
    const fullPath = path.join(logDir, logFile);
    const content = fs.readFileSync(fullPath, 'utf8');
    res.type('text/plain').send(content);
});
