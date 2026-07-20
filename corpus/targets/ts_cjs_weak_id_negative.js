var crypto = require('crypto');

function createTenant(name, ownerId) {
    var tenantId = 'tnt_' + crypto.randomUUID();
    return { id: tenantId, name: name, ownerId: ownerId };
}

function generateSessionToken() {
    var bytes = crypto.randomBytes(32);
    return bytes.toString('hex');
}

function createInviteCode() {
    return 'inv_' + crypto.randomUUID();
}

function generateApiKey(userId) {
    var secret = crypto.randomUUID().replace(/-/g, '');
    return 'fhp_' + userId.slice(0, 8) + '_' + secret;
}
