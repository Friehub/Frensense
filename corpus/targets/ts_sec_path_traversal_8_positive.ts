const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.post('/save-config', (req, res) => {
    const { section, key, value } = req.body;
    const configPath = path.join('/etc/app/configs', section, `${key}.json`);
    fs.mkdirSync(path.dirname(configPath), { recursive: true });
    fs.writeFileSync(configPath, JSON.stringify(value));
    res.json({ saved: configPath });
});
