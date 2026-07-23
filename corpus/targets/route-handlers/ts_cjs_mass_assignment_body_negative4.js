// SAFE: Update uses a predefined schema that only maps specific fields
var express = require('express');

var profileSchema = {
    displayName: true,
    email: true,
    bio: true,
    avatarUrl: true
};

function applySchema(body, schema) {
    var result = {};
    Object.keys(schema).forEach(function(field) {
        if (body[field] !== undefined) {
            result[field] = body[field];
        }
    });
    return result;
}

module.exports = function(app, db) {
    app.put('/api/profile', function(req, res) {
        var updates = applySchema(req.body, profileSchema);
        db.collection('users').updateOne(
            { _id: req.session.userId },
            { $set: updates },
            function(err, result) {
                if (err) return res.status(500).json({ error: 'Update failed' });
                res.json({ success: true });
            }
        );
    });
};
