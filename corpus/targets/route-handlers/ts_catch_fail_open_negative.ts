export async function checkAndConsumeQuota(userId: string) {
  try {
    const res = await fetch("https://api.internal/quota", { method: "POST", body: JSON.stringify({ userId }) });
    if (!res.ok) throw new Error("Quota check failed");
    return { allowed: true, remaining: 10 };
  } catch (e: any) {
    console.error("checkAndConsumeQuota failed:", e);
    // SAFE: Failing closed on error
    return { allowed: false, reason: "quota_service_unavailable" };
  }
}

export async function expressHandler(req: any, res: any, next: any) {
  try {
    const data = await db.query("SELECT * FROM users");
    res.json(data);
  } catch (e) {
    // SAFE: Passing error to middleware
    next(e);
  }
}
