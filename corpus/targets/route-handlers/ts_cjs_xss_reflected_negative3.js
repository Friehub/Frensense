// SAFE: Uses EJS template engine which auto-escapes HTML in variables
var express = require('express');
var app = express();

app.set('view engine', 'ejs');

function profileHandler(req, res) {
    var username = req.query.username;
    res.render('profile', { username: username });
}

function commentHandler(req, res) {
    var comment = req.body.text;
    res.render('comment', { text: comment });
}

app.get('/profile', profileHandler);
app.post('/comment', commentHandler);
