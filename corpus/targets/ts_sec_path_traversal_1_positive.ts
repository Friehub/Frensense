const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.get('/files/:filename', (req, res) => {
    const filePath = path.join('/data/uploads', req.params.filename);
    const content = fs.readFileSync(filePath, 'utf8');
    res.send(content);
});
