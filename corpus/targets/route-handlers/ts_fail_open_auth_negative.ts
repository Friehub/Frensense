// [frensense]
// observation: An error handling block for an authorization or quota check safely fails closed.
// impact: None — security and quotas are enforced even when downstream systems are unavailable.
// improvement: N/A — this is the correct pattern.

export async function checkAndConsumeQuota(env: any, userId: string): Promise<{ allowed: boolean, remaining?: number, reason?: string }> {
    try {
        const res = await fetch(`http://internal-billing/check/${userId}`);
        if (!res.ok) throw new Error("Billing down");
        const data = await res.json();
        return { allowed: data.remaining > 0, remaining: data.remaining };
    } catch (e: any) {
        console.error("Quota check failed:", e);
        // Good: fail closed in production, only allow bypass in dev
        if (env.ENVIRONMENT === "development") {
            return { allowed: true, remaining: 999 };
        }
        return { allowed: false, reason: "quota_service_unavailable" };
    }
}

export async function verifyUserPermission(token: string): Promise<boolean> {
    try {
        const decoded = await verifyToken(token);
        return decoded.role === 'admin';
    } catch (error) {
        // Good: fail closed
        return false;
    }
}
