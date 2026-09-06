// SAFE: Uses crypto.randomUUID() for secure, collision-resistant user IDs
var crypto = require('crypto');

function createUser(name, email) {
    var id = crypto.randomUUID();
    return { id: id, name: name, email: email };
}

function generateOrderId() {
    return 'ord_' + crypto.randomUUID();
}

function createPaymentReference() {
    return 'pay_' + crypto.randomUUID().replace(/-/g, '');
}
