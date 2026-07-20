// SAFE: Uses crypto.randomUUID() for all unique identifiers
var crypto = require('crypto');

function generateApiToken() {
    return 'fhp_' + crypto.randomUUID().replace(/-/g, '');
}

function createResetToken(userId) {
    return userId + '_' + crypto.randomUUID();
}

function generateSessionId() {
    return crypto.randomUUID();
}
