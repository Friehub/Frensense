// [frensense]
// observation: The application attaches its own internal access token or credential to an HTTP request sent to a URL that may not be fully trusted or is user-controlled.
// impact: If the URL points to an attacker-controlled server, the attacker will receive the internal token in the Authorization header, leading to credential theft and potential unauthorized access to your infrastructure.
// improvement: Validate the destination URL against an allowlist of trusted internal domains before attaching internal credentials to the request.

export async function proxySandboxRequest(url: string, token: string): Promise<Response> {
    // Bad: Attaches our own token to an arbitrary URL
    const headers: Record<string, string> = {
        "Authorization": `Bearer ${token}`
    };
    return await fetch(url, { headers });
}

export async function notifyWebhook(payload: any, webhookUrl: string, apiSecret: string) {
    // Bad: sending our secret to a user-provided webhook
    return await fetch(webhookUrl, {
        method: "POST",
        headers: { "X-Api-Key": apiSecret },
        body: JSON.stringify(payload)
    });
}
