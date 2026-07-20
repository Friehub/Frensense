var crypto = require('crypto');

function generateCsrfToken() {
    return crypto.randomUUID();
}

function createPasswordResetToken(userId) {
    var token = crypto.randomUUID().replace(/-/g, '');
    var buf = crypto.randomBytes(16).toString('hex');
    return userId + '_' + buf;
}

function generateNonce() {
    return crypto.randomUUID();
}
