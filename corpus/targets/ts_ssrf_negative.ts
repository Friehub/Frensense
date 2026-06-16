import { URL } from "url";

const ALLOWED_HOSTS = ["api.internal.com", "data.internal.com"];

function isAllowedUrl(urlStr: string): boolean {
    try {
        const parsed = new URL(urlStr);
        return ALLOWED_HOSTS.includes(parsed.hostname) && parsed.protocol === "https:";
    } catch {
        return false;
    }
}

async function fetchUserData(req: Request, res: Response) {
    const url = req.query.url;
    if (!isAllowedUrl(url)) {
        return res.status(403).json({ error: "URL not allowed" });
    }
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = req.body.target;
    if (!isAllowedUrl(target)) {
        return res.status(403).json({ error: "Target not allowed" });
    }
    const result = await fetch(target, {
        method: req.body.method,
        headers: req.body.headers,
    });
    const body = await result.text();
    res.send(body);
}

async function loadWebhook(req: Request, res: Response) {
    const webhookUrl = req.params.url;
    if (!isAllowedUrl(webhookUrl)) {
        return res.status(403).json({ error: "Webhook URL not allowed" });
    }
    const resp = await fetch(webhookUrl);
    res.json({ status: resp.status });
}
