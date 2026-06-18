const { exec } = require('child_process');
const express = require('express');
const app = express();

app.get('/stats', (req, res) => {
    const metric = req.query.metric;
    const period = req.query.period;
    const cmd = `collectd --query "${metric}" --since "${period}"`;
    exec(cmd, (err, stdout) => {
        res.type('application/json').send(stdout);
    });
});
