// [frensense]
// observation: An external webhook endpoint processes incoming payloads without verifying cryptographic signatures.
// impact: Anyone who discovers the webhook URL can forge events (e.g., faking a successful Stripe payment) to gain unauthorized services.
// improvement: Verify the webhook payload against a known signing secret before processing the event.

export async function POST(req: Request) {
  // VULNERABLE: parses and processes webhook blindly
  const payload = await req.json();
  
  if (payload.type === 'checkout.session.completed') {
    await fulfillOrder(payload.data.object.client_reference_id);
    return new Response('Success', { status: 200 });
  }

  return new Response('Ignored', { status: 200 });
}

app.post('/webhook/github', async (req, res) => {
  // VULNERABLE: no verification of X-Hub-Signature-256
  const event = req.headers['x-github-event'];
  if (event === 'push') {
    await triggerDeploy(req.body.repository.id);
  }
  res.send('OK');
});
