// SAFE: Feature flags are enforced server-side based on subscription data
export async function uploadLargeFile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const sub = await db.prepare('SELECT * FROM subscriptions WHERE user_id = ? AND active = 1').bind(session.userId).first();
  const maxSize = sub?.tier === 'premium' ? 1024 * 1024 * 1024 : 100 * 1024 * 1024;
  if (req.body.file.size > maxSize) {
    return new Response('Upgrade to premium for larger files', { status: 402 });
  }
  await uploadToS3(req.body.file);
  return new Response('Uploaded');
}

export async function generateReport(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const sub = await db.prepare('SELECT * FROM subscriptions WHERE user_id = ? AND active = 1').bind(session.userId).first();
  if (!sub || sub.tier === 'free') return new Response('Premium feature', { status: 402 });
  const data = await fetchReportData(req.body.params);
  return new Response(JSON.stringify(data));
}
