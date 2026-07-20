// [frensense]
// observation: Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load.
// impact: Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing.
// improvement: Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens.

var express = require('express');
var app = express();

function createTenant(name, ownerId) {
    // VULNERABLE: millisecond resolution — collides under concurrent load
    var tenantId = 'tnt_' + Date.now();
    return { id: tenantId, name: name, ownerId: ownerId };
}

function generateSessionToken() {
    // VULNERABLE: Math.random is not cryptographically secure
    return Math.random().toString(36).slice(2);
}

function createInviteCode() {
    // VULNERABLE: timestamp-based codes are guessable
    return 'inv_' + new Date().getTime();
}

function generateApiKey(userId) {
    // VULNERABLE: predictable key
    return 'key_' + userId + '_' + Date.now();
}

app.post('/tenant', function(req, res) {
    var tenant = createTenant(req.body.name, req.body.ownerId);
    res.json(tenant);
});
