const { exec } = require('child_process');
const express = require('express');
const app = express();

app.post('/process', (req, res) => {
    const { name, action } = req.body;
    exec(`./worker.sh ${action} ${name}`, (err, stdout) => {
        res.json({ result: stdout });
    });
});
