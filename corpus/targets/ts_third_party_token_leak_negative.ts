// [frensense]
// observation: The application validates the destination URL against an allowlist before attaching sensitive credentials.
// impact: None — credentials are only sent to trusted internal endpoints.
// improvement: N/A — this is the correct pattern.

export async function proxySandboxRequest(url: string, token: string): Promise<Response> {
    const parsedUrl = new URL(url);
    const headers: Record<string, string> = {};
    
    // Good: Only attach token if the destination is our own trusted domain
    if (parsedUrl.hostname.endsWith(".friehub.cloud") || parsedUrl.hostname === "vercel-sandbox-bridge.vercel.app") {
        headers["Authorization"] = `Bearer ${token}`;
    }
    
    return await fetch(url, { headers });
}

export async function notifyWebhook(payload: any, webhookUrl: string, apiSecret: string) {
    const parsed = new URL(webhookUrl);
    const headers: Record<string, string> = {};
    
    // Good: only send secret to our internal workers
    if (parsed.hostname === "internal-worker.local") {
        headers["X-Api-Key"] = apiSecret;
    }
    
    return await fetch(webhookUrl, {
        method: "POST",
        headers,
        body: JSON.stringify(payload)
    });
}
