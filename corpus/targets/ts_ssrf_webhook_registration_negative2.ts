// SAFE: Webhooks are dispatched through a server-side webhook dispatch service that validates URLs before each call
import express from "express";

class WebhookDispatcher {
    private allowedHosts = new Set(["hooks.slack.com", "events.internal.com"]);

    async send(url: string, payload: any): Promise<boolean> {
        try {
            const parsed = new URL(url);
            if (parsed.protocol !== "https:") return false;
            if (!this.allowedHosts.has(parsed.hostname)) return false;
            const resp = await fetch(url, {
                method: "POST",
                body: JSON.stringify(payload),
                headers: { "Content-Type": "application/json" },
                signal: AbortSignal.timeout(5000),
            });
            return resp.ok;
        } catch {
            return false;
        }
    }
}

const dispatcher = new WebhookDispatcher();

export async function triggerWebhooks(event: string, payload: any) {
    const webhooks = await db.query("SELECT url FROM webhooks WHERE event = ?", [event]);
    const results = await Promise.allSettled(
        webhooks.map((w: any) => dispatcher.send(w.url, payload))
    );
}
