// [frensense]
// observation: An HTTP request is made to a potentially dynamic URL, but no sensitive authentication tokens or credentials are included in the headers.
// impact: None — since no credentials are sent, an attacker cannot steal internal tokens even if they control the destination URL.
// improvement: N/A — this is the correct pattern.

export async function proxyPublicRequest(url: string, userAgent: string): Promise<Response> {
    // Good: Sending harmless headers like User-Agent or Content-Type is safe
    const headers: Record<string, string> = {
        "User-Agent": userAgent,
        "Accept": "application/json"
    };
    return await fetch(url, { headers });
}

export async function forwardPayload(payload: any, targetUrl: string) {
    // Good: Content-Type is not a secret
    return await fetch(targetUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload)
    });
}
