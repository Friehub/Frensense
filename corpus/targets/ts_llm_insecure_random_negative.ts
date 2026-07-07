import crypto from 'crypto';

function generateResetToken() {
    return crypto.randomBytes(24).toString('hex');
}
