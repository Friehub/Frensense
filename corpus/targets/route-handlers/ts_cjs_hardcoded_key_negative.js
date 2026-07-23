var express = require('express');
var app = express();

var API_KEY = process.env.API_KEY;
var DB_PASSWORD = process.env.DB_PASSWORD;

function getData(req, res) {
    res.json({ status: "connected" });
}

app.get('/data', getData);
