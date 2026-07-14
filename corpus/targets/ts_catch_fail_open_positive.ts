export async function checkAndConsumeQuota(userId: string) {
  try {
    const res = await fetch("https://api.internal/quota", { method: "POST", body: JSON.stringify({ userId }) });
    if (!res.ok) throw new Error("Quota check failed");
    return { allowed: true, remaining: 10 };
  } catch (e: any) {
    console.error("checkAndConsumeQuota failed:", e);
    // VULNERABILITY: Failing open on error, bypassing quota
    return { allowed: true, remaining: 999 };
  }
}
