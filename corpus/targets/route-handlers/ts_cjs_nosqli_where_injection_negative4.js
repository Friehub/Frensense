// SAFE: Validates userId is a valid ObjectId before using in MongoDB query
var express = require('express');
var ObjectId = require('mongodb').ObjectId;

module.exports = function(app, db) {
    app.get('/api/user/:userId/profile', function(req, res) {
        var userId = req.params.userId;
        if (!ObjectId.isValid(userId)) {
            return res.status(400).json({ error: 'Invalid user ID' });
        }
        db.collection('profiles').findOne({
            _id: ObjectId(userId)
        }, function(err, profile) {
            if (err) return res.status(500).json({ error: 'Query failed' });
            if (!profile) return res.status(404).json({ error: 'Not found' });
            res.json(profile);
        });
    });

    app.get('/api/user/:userId/settings', function(req, res) {
        var userId = req.params.userId;
        if (!ObjectId.isValid(userId)) {
            return res.status(400).json({ error: 'Invalid user ID' });
        }
        db.collection('settings').findOne({
            _id: ObjectId(userId)
        }, function(err, settings) {
            if (err) return res.status(500).json({ error: 'Query failed' });
            if (!settings) return res.status(404).json({ error: 'Not found' });
            res.json(settings);
        });
    });
};
