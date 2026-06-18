const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.post('/export', (req, res) => {
    const { dir, filename } = req.body;
    const source = path.join('/data/reports', dir, filename);
    const dest = path.join('/tmp/exports', filename);
    fs.copyFileSync(source, dest);
    res.json({ exported: dest });
});
