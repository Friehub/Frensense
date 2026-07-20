// SAFE: Uses crypto.randomBytes(32).toString('hex') for cryptographically secure token generation
var crypto = require('crypto');

function generateApiToken() {
    return 'fhp_' + crypto.randomBytes(32).toString('hex');
}

function createResetToken(userId) {
    var raw = crypto.randomBytes(24).toString('hex');
    return userId + '_' + raw;
}

function generateSessionId() {
    return crypto.randomBytes(16).toString('hex');
}
