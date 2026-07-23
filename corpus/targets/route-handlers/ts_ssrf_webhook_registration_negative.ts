// SAFE: Webhook URL is validated against an allowlist at registration time
import express from "express";

const ALLOWED_WEBHOOK_HOSTS = new Set([
    "hooks.slack.com",
    "hooks.example.com",
    "events.internal.com",
]);

function isValidUrl(url: string): boolean {
    try {
        const parsed = new URL(url);
        return parsed.protocol === "https:" && ALLOWED_WEBHOOK_HOSTS.has(parsed.hostname);
    } catch {
        return false;
    }
}

export async function registerWebhook(req: express.Request, res: express.Response) {
    const { url, event } = req.body;
    if (!isValidUrl(url)) {
        return res.status(400).json({ error: "Invalid webhook URL" });
    }
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
            signal: AbortSignal.timeout(5000),
        });
    }
}
