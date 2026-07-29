// SAFE: Set X-Content-Type-Options: nosniff header.

const express = require("express");
const app = express();

app.use((req, res, next) => {
    res.setHeader("X-Content-Type-Options", "nosniff");
    next();
});

app.get("/", (req, res) => {
    res.send("Hello");
});