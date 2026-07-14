// SAFE: Webhook payload verified against cryptographic signature
import Stripe from 'stripe';
const stripe = new Stripe(process.env.STRIPE_SECRET_KEY!);

export async function POST(req: Request) {
  const signature = req.headers.get('stripe-signature');
  if (!signature) return new Response('No signature', { status: 401 });

  const rawBody = await req.text();
  let event;

  try {
    // SAFE: Stripe constructEvent verifies the payload against the webhook secret
    event = stripe.webhooks.constructEvent(rawBody, signature, process.env.STRIPE_WEBHOOK_SECRET!);
  } catch (err) {
    return new Response(`Webhook Error: ${err.message}`, { status: 400 });
  }

  if (event.type === 'checkout.session.completed') {
    await fulfillOrder(event.data.object.client_reference_id);
    return new Response('Success', { status: 200 });
  }

  return new Response('Ignored', { status: 200 });
}

app.post('/webhook/github', express.raw({ type: 'application/json' }), async (req, res) => {
  const signature = req.headers['x-hub-signature-256'];
  // SAFE: verifies HMAC digest
  const expected = 'sha256=' + crypto.createHmac('sha256', process.env.GITHUB_SECRET)
                                    .update(req.body)
                                    .digest('hex');
  if (signature !== expected) return res.status(401).send('Bad signature');

  const event = req.headers['x-github-event'];
  if (event === 'push') {
    await triggerDeploy(JSON.parse(req.body).repository.id);
  }
  res.send('OK');
});
