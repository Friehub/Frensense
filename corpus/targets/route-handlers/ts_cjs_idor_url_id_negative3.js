// SAFE: SQL query includes AND user_id = ? to verify resource ownership
var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function getSession(req) {
    return { userId: req.session.userId };
}

app.get('/document/:docId', function(req, res) {
    var docId = req.params.docId;
    var session = getSession(req);
    connection.query('SELECT * FROM documents WHERE id = ? AND user_id = ?', [docId, session.userId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
});

app.get('/note/:noteId', function(req, res) {
    var noteId = req.params.noteId;
    var session = getSession(req);
    connection.query('SELECT * FROM notes WHERE id = ? AND user_id = ?', [noteId, session.userId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
});
