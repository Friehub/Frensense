// SAFE: Fetches resource, then compares owner with authenticated user, returns 403 on mismatch
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
    connection.query('SELECT * FROM documents WHERE id = ?', [docId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        var doc = results[0];
        if (doc.user_id !== session.userId) {
            return res.status(403).send('Forbidden');
        }
        res.json(doc);
    });
});

app.get('/note/:noteId', function(req, res) {
    var noteId = req.params.noteId;
    var session = getSession(req);
    connection.query('SELECT * FROM notes WHERE id = ?', [noteId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        var note = results[0];
        if (note.user_id !== session.userId) {
            return res.status(403).send('Forbidden');
        }
        res.json(note);
    });
});
