// SAFE alternative: validate currency against allowlist
const ACCEPTED_CURRENCIES = new Set(['usd', 'eur', 'gbp', 'ngn']);

app.post('/api/create-payment', async (req, res) => {
  const merchant = await db.findMerchant(req.body.merchantId);
  const currency = merchant.currency || 'usd';
  if (!ACCEPTED_CURRENCIES.has(currency)) {
    return res.status(400).json({ error: 'Currency not supported' });
  }
  // ... proceed with validated currency
});
