// SAFE: Uses lodash pick to only allow specific fields from req.body
var _ = require('lodash');
var express = require('express');

var allowedFields = ['displayName', 'email', 'bio'];

module.exports = function(app, db) {
    app.put('/api/profile', function(req, res) {
        var updates = _.pick(req.body, allowedFields);
        db.collection('users').updateOne(
            { _id: req.session.userId },
            { $set: updates },
            function(err, result) {
                if (err) return res.status(500).json({ error: 'Update failed' });
                res.json({ success: true });
            }
        );
    });

    app.put('/api/settings', function(req, res) {
        var updates = _.pick(req.body, ['theme', 'locale', 'notifications']);
        db.collection('settings').updateOne(
            { _id: req.session.userId },
            { $set: updates },
            function(err, result) {
                if (err) return res.status(500).json({ error: 'Update failed' });
                res.json({ success: true });
            }
        );
    });
};
