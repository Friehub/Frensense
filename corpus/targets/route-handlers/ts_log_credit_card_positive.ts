// [frensense]
// observation: Credit card number, CVV, or full PAN logged in plain text.
// impact: Logging full PAN (Primary Account Number) violates PCI-DSS Requirement 10. Non-compliance results in fines ($50k-$500k/month), loss of payment processing privileges, or mandated forensic audits.
// improvement: Never log full card numbers. If needed for debugging, log only the last 4 digits. Mask or truncate all card data before logging.

import { logger } from './logger';

app.post('/api/payment', async (req, res) => {
  // VULNERABLE: credit card number logged
  logger.info('Payment processed', {
    cardNumber: req.body.cardNumber,
    cvv: req.body.cvv,
    expiry: req.body.expiry,
    amount: req.body.amount,
  });
  // ...
});

app.post('/api/refund', async (req, res) => {
  // VULNERABLE: card number in plaintext log
  console.log('Processing refund for card:', req.body.lastFour, 'full:', req.body.cardNumber);
  // ...
});
