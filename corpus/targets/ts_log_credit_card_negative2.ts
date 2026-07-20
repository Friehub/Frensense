// SAFE alternative: never log card details, use token
app.post('/api/payment', async (req, res) => {
  // SAFE: only payment intent ID logged, no card data
  logger.info('Payment processed', {
    paymentIntentId: paymentIntent.id,
    amount: paymentIntent.amount,
    status: paymentIntent.status,
  });
});
