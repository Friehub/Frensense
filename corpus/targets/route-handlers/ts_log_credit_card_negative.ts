// SAFE: mask card data before logging
function maskCardNumber(card: string): string {
  return card.replace(/\d(?=\d{4})/g, '*');
}

app.post('/api/payment', async (req, res) => {
  // SAFE: only log last 4 digits
  logger.info('Payment processed', {
    cardLastFour: req.body.cardNumber?.slice(-4),
    expiry: req.body.expiry,
    amount: req.body.amount,
    masked: maskCardNumber(req.body.cardNumber),
  });
});

app.post('/api/refund', async (req, res) => {
  // SAFE: only log tokenized or masked data
  logger.info('Refund processed', {
    cardLastFour: req.body.cardNumber?.slice(-4),
    amount: req.body.amount,
  });
});
