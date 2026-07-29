// SAFE: Disable X-Powered-By header.

const express = require("express");
const app = express();

app.disable("x-powered-by");

app.get("/", (req, res) => {
    res.send("Hello");
});