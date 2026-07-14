// [frensense]
// observation: An HTTP request is made to a URL read from application configuration, not from user input.
// impact: None — this is a safe outbound call because the URL is hardcoded in config.
// improvement: N/A — this is the correct pattern.

const API_CONFIG = {
    dataEndpoint: "https://api.internal.com/v1/data",
    metricsEndpoint: "https://metrics.internal.com/ingest",
    webhookUrl: "https://hooks.internal.com/notify",
};

async function syncMetrics(env: any): Promise<void> {
    const response = await fetch(API_CONFIG.metricsEndpoint, {
        method: "POST",
        body: JSON.stringify({ timestamp: Date.now() }),
    });
    if (!response.ok) {
        throw new Error(`Metrics sync failed: ${response.status}`);
    }
}

async function notifyWebhook(payload: object): Promise<void> {
    await fetch(API_CONFIG.webhookUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
    });
}
