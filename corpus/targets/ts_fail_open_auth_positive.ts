// [frensense]
// observation: An error handling block for an authorization or quota check returns a success/allowed result.
// impact: A fail-open authorization mechanism allows users to bypass security or billing limits if the downstream service is temporarily unavailable, leading to abuse.
// improvement: Fail closed in production. If the authorization service is down, deny the request (return false or allowed: false) to maintain security and quotas.

export async function checkAndConsumeQuota(userId: string): Promise<{ allowed: boolean, remaining?: number }> {
    try {
        const res = await fetch(`http://internal-billing/check/${userId}`);
        if (!res.ok) throw new Error("Billing down");
        const data = await res.json();
        return { allowed: data.remaining > 0, remaining: data.remaining };
    } catch (e: any) {
        console.error("Quota check failed:", e);
        // Bad: failing open for "dev purposes" but this runs in production
        return { allowed: true, remaining: 999 };
    }
}

export async function verifyUserPermission(token: string): Promise<boolean> {
    try {
        const decoded = await verifyToken(token);
        return decoded.role === 'admin';
    } catch (error) {
        // Bad: failing open if token verification crashes
        return true;
    }
}
