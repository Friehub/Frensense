const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.post('/download', (req, res) => {
    const file = req.query.path;
    const fullPath = path.join('/data/public', file);
    fs.readFile(fullPath, (err, data) => {
        if (err) return res.status(404).send('not found');
        res.send(data);
    });
});
