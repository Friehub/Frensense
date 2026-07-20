// SAFE: check event timestamp with tolerance window
const MAX_AGE_MS = 5 * 60 * 1000; // 5 minutes

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  const eventTime = event.created * 1000; // Stripe uses Unix timestamps
  const age = Date.now() - eventTime;
  if (Math.abs(age) > MAX_AGE_MS) {
    return res.status(400).json({ error: 'Event too old' });
  }
  if (event.type === 'checkout.session.completed') {
    await fulfillOrder(event.data.object.id);
  }
  res.json({ received: true });
});

app.post('/webhooks/github', async (req, res) => {
  const event = req.body;
  const eventTime = new Date(event.head_commit?.timestamp || event.repository?.pushed_at).getTime();
  if (Date.now() - eventTime > MAX_AGE_MS) {
    return res.status(400).json({ error: 'Event too old' });
  }
  if (event.action === 'published' && event.release) {
    await deployRelease(event.release.tag_name);
  }
  res.json({ received: true });
});
