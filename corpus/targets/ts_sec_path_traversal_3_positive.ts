const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.post('/backup', (req, res) => {
    const dest = req.body.destination;
    const src = '/data/db.sqlite';
    const target = path.join('/backups', dest);
    fs.copyFileSync(src, target);
    res.json({ backed_up: target });
});
