const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

function resolveTemplate(name) {
    return path.join('/templates', name);
}

app.get('/template', (req, res) => {
    const templatePath = resolveTemplate(req.query.name);
    const content = fs.readFileSync(templatePath, 'utf8');
    res.type('text/html').send(content);
});
