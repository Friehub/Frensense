const express = require('express');
const fs = require('fs');
const path = require('path');
const app = express();

app.get('/icon', (req, res) => {
    const icon = req.headers['x-icon-name'];
    const iconPath = path.join('/static/icons', icon);
    res.sendFile(iconPath);
});
