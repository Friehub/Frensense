// SAFE: Uses nanoid for compact, collision-resistant, cryptographically secure IDs
var { nanoid } = require('nanoid');

function createTenant(name, ownerId) {
    var tenantId = 'tnt_' + nanoid(16);
    return { id: tenantId, name: name, ownerId: ownerId };
}

function generateSessionToken() {
    return nanoid(32);
}

function createInviteCode() {
    return 'inv_' + nanoid(12);
}

function generateApiKey(userId) {
    return 'fhp_' + userId.slice(0, 8) + '_' + nanoid(24);
}
