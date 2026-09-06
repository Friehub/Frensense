// SAFE: Use HTTPS to encrypt traffic.

const express = require("express");
const https = require("https");
const fs = require("fs");
const app = express();

const httpsOptions = {
    key: fs.readFileSync("./server.key"),
    cert: fs.readFileSync("./server.crt")
};

app.get("/", (req, res) => {
    res.send("Hello");
});

https.createServer(httpsOptions, app).listen(443);