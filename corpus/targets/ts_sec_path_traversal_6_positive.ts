const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.post('/upload', (req, res) => {
    const { folder, filename, content } = req.body;
    const dest = path.join('/data/uploads', folder, filename);
    fs.writeFileSync(dest, content);
    res.json({ saved: dest });
});
