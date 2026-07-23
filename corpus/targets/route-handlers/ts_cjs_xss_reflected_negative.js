var express = require('express');
var app = express();

function escapeHtml(str) {
    return String(str)
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#x27;");
}

function searchHandler(req, res) {
    var query = escapeHtml(req.query.q);
    res.send("<html><body><h1>Search results for: " + query + "</h1></body></html>");
}

function greetingHandler(req, res) {
    var name = escapeHtml(req.query.name);
    res.send("<p>Welcome, " + name + "!</p>");
}

app.get('/search', searchHandler);
app.get('/greet', greetingHandler);
