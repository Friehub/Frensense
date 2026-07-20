// SAFE: crypto.randomBytes for raw entropy
var crypto = require('crypto');

function generateCsrfToken() {
    return crypto.randomBytes(32).toString('hex');
}

function createPasswordResetToken(userId) {
    return userId + '_' + crypto.randomBytes(24).toString('base64url');
}
