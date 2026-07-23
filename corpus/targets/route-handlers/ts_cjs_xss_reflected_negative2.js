// SAFE: Uses EJS template engine with auto-escaping
var express = require('express');
var app = express();

app.set("view engine", "ejs");

function searchHandler(req, res) {
    var query = req.query.q;
    res.render("search", { query: query });
}

function greetingHandler(req, res) {
    var name = req.query.name;
    res.render("greeting", { name: name });
}

app.get('/search', searchHandler);
app.get('/greet', greetingHandler);
