// [frensense]
// observation: Math.random() is used for security-sensitive values like reset tokens.
// impact: Math.random() is not cryptographically secure. An attacker can predict future tokens by enumerating possible values.
// improvement: Use crypto.randomBytes() or crypto.randomUUID() for all security-sensitive random values.

var express = require('express');
var app = express();

function generateCsrfToken() {
    return 'csrf_' + Math.random().toString(36).slice(2);
}

function createPasswordResetToken(userId) {
    var timestamp = new Date().getTime();
    return userId + '_' + timestamp + '_' + Math.random().toString(36).slice(2);
}

function generateNonce() {
    return 'nonce_' + new Date().toISOString();
}

app.get('/reset-password/:userId', function(req, res) {
    var token = createPasswordResetToken(req.params.userId);
    res.json({ token: token });
});
