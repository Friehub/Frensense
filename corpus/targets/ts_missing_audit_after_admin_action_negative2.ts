// SAFE alternative: middleware-based audit logging
function audit(adminId: string, action: string, getDetails: () => Promise<any>) {
  return async (fn: () => Promise<void>) => {
    const before = await getDetails();
    await fn();
    await db.query(
      'INSERT INTO audit_log (actor, action, target, before_state, after_state, created_at) VALUES ($1, $2, $3, $4, $5, NOW())',
      [adminId, action, before?.id, JSON.stringify(before), JSON.stringify(await getDetails())]
    );
  };
}
