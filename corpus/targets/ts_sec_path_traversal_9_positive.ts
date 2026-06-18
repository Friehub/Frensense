const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

async function readFile(name) {
    const fullPath = path.join('/data/docs', name);
    return fs.promises.readFile(fullPath, 'utf8');
}

app.get('/doc', async (req, res) => {
    const content = await readFile(req.query.name);
    res.type('text/markdown').send(content);
});
