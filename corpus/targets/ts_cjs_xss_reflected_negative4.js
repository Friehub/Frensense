// SAFE: User input is HTML-entity encoded before being sent in the response
var express = require('express');
var app = express();

function escapeHtml(str) {
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

function profileHandler(req, res) {
    var username = escapeHtml(req.query.username);
    res.send('<div class="profile">Welcome, ' + username + '!</div>');
}

function commentHandler(req, res) {
    var text = escapeHtml(req.body.text);
    res.send('<p class="comment">' + text + '</p>');
}

app.get('/profile', profileHandler);
app.post('/comment', commentHandler);
