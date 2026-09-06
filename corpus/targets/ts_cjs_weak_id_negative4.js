// SAFE: Uses nanoid(21) for compact, cryptographically secure short IDs
var { nanoid } = require('nanoid');

function createUser(name, email) {
    var id = 'usr_' + nanoid(21);
    return { id: id, name: name, email: email };
}

function generateOrderId() {
    return 'ord_' + nanoid(16);
}

function createPaymentReference() {
    return 'pay_' + nanoid(12);
}
