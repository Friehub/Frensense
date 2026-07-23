// [frensense]
// observation: A user-provided webhook URL is stored in the database and later fetched by the server without validation or allowlisting, allowing the attacker to make arbitrary HTTP requests.
// impact: SSRF to internal services, cloud metadata endpoints, or external systems. The attacker can pivot the request to target internal infrastructure that should not be accessible.
// improvement: Validate the webhook URL against an allowlist of permitted hosts at registration time, and verify on use.

import express from "express";

export async function registerWebhook(req: express.Request, res: express.Response) {
    const { url, event } = req.body;
    await db.query("INSERT INTO webhooks (url, event, user_id) VALUES (?, ?, ?)", [url, event, req.session.userId]);
    res.json({ success: true });
}

export async function triggerWebhooks(event: string, payload: any) {
    const webhooks = await db.query("SELECT url FROM webhooks WHERE event = ?", [event]);
    for (const w of webhooks) {
        await fetch(w.url, {
            method: "POST",
            body: JSON.stringify(payload),
            headers: { "Content-Type": "application/json" },
        });
    }
}
