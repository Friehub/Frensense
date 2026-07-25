// [frensense]
// observation: Integer input accepted without minimum or maximum range validation.
// impact: Negative or overflow values can cause logical errors (negative price, infinite loops), financial loss, or security bypasses. For example, setting age to -1 or quantity to 999999999 during checkout.
// improvement: Validate integer inputs with explicit min/max bounds. Use zod, express-validator, or manual range checks.
// cwe: CWE-190
// cvss: 7.5
// owasp: 
// severity: High

app.post('/api/checkout', async (req, res) => {
  // VULNERABLE: quantity not range-checked
  const { productId, quantity } = req.body;
  const product = await db.findProduct(productId);

  const total = product.price * quantity; // overflow or negative
  await createOrder(req.user.id, productId, quantity, total);
  res.json({ total });
});

app.post('/api/profile', async (req, res) => {
  // VULNERABLE: age not range-checked
  const { age } = req.body;
  await db.query('UPDATE users SET age = $1 WHERE id = $2', [age, req.user.id]);
  res.json({ status: 'ok' });
});
