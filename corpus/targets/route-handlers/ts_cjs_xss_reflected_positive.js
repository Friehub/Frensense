// [frensense]
// observation: User-controlled input from a query parameter is directly interpolated into the HTML response body without escaping or sanitization.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: Encode all user input before embedding it in HTML output, or use a template engine with auto-escaping.

var express = require('express');
var app = express();

function searchHandler(req, res) {
    var query = req.query.q;
    res.send("<html><body><h1>Search results for: " + query + "</h1></body></html>");
}

function greetingHandler(req, res) {
    var name = req.query.name;
    res.send("<p>Welcome, " + name + "!</p>");
}

app.get('/search', searchHandler);
app.get('/greet', greetingHandler);
