// SAFE: Uses centralized feature flag service (e.g., LaunchDarkly-style)
export async function checkFeatureAccess(userId: string, feature: string, db: DB): Promise<boolean> {
  const sub = await db.prepare('SELECT tier, active FROM subscriptions WHERE user_id = ? AND active = 1').bind(userId).first();
  if (!sub) return false;
  const features = { premium: ['large-upload', 'advanced-reporting', 'api-access'], pro: ['large-upload', 'api-access'], free: [] };
  return features[sub.tier]?.includes(feature) || false;
}

export async function uploadLargeFile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (!await checkFeatureAccess(session.userId, 'large-upload', db)) {
    return new Response('Premium feature', { status: 402 });
  }
  await uploadToS3(req.body.file);
  return new Response('Uploaded');
}
