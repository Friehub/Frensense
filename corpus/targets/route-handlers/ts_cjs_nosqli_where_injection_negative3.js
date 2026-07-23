// SAFE: MongoDB query uses findOne with ownership filter instead of $where
var express = require('express');
var ObjectId = require('mongodb').ObjectId;

module.exports = function(app, db) {
    app.get('/api/document/:docId', function(req, res) {
        var docId = req.params.docId;
        var sessionUserId = req.session.userId;
        if (!ObjectId.isValid(docId)) {
            return res.status(400).json({ error: 'Invalid document ID' });
        }
        db.collection('documents').findOne({
            _id: ObjectId(docId),
            ownerId: sessionUserId
        }, function(err, doc) {
            if (err) return res.status(500).json({ error: 'Query failed' });
            if (!doc) return res.status(404).json({ error: 'Not found' });
            res.json(doc);
        });
    });

    app.get('/api/note/:noteId', function(req, res) {
        var noteId = req.params.noteId;
        var sessionUserId = req.session.userId;
        if (!ObjectId.isValid(noteId)) {
            return res.status(400).json({ error: 'Invalid note ID' });
        }
        db.collection('notes').findOne({
            _id: ObjectId(noteId),
            ownerId: sessionUserId
        }, function(err, note) {
            if (err) return res.status(500).json({ error: 'Query failed' });
            if (!note) return res.status(404).json({ error: 'Not found' });
            res.json(note);
        });
    });
};
