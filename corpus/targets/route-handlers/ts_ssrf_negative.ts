import { URL } from "url";

const ALLOWED_HOSTS = new Set([
    "api.internal.com",
    "data.internal.com",
    "cdn.trusted.com",
]);

const BLOCKED_RANGES = [
    /^127\./,
    /^10\./,
    /^172\.(1[6-9]|2\d|3[01])\./,
    /^192\.168\./,
    /^0\./,
    /^localhost$/i,
];

function isUrlSafe(urlStr: string): { safe: boolean; reason?: string } {
    try {
        const parsed = new URL(urlStr);

        if (!["http:", "https:"].includes(parsed.protocol)) {
            return { safe: false, reason: "Only HTTP(S) allowed" };
        }

        const hostname = parsed.hostname.toLowerCase();

        for (const blocked of BLOCKED_RANGES) {
            if (blocked.test(hostname)) {
                return { safe: false, reason: "Internal IP address blocked" };
            }
        }

        if (!ALLOWED_HOSTS.has(hostname)) {
            return { safe: false, reason: `Host ${hostname} not in allowlist` };
        }

        if (parsed.port && parseInt(parsed.port, 10) > 65535) {
            return { safe: false, reason: "Invalid port" };
        }

        const pathname = decodeURIComponent(parsed.pathname);
        if (pathname.includes("..") || pathname.includes("~")) {
            return { safe: false, reason: "Path traversal detected" };
        }

        return { safe: true };
    } catch {
        return { safe: false, reason: "Invalid URL format" };
    }
}

async function fetchWithValidation(url: string): Promise<Response> {
    const check = isUrlSafe(url);
    if (!check.safe) {
        throw new Error(`URL validation failed: ${check.reason}`);
    }
    return fetch(url, {
        method: "GET",
        headers: { "User-Agent": "FrensenseBot/1.0" },
        signal: AbortSignal.timeout(5000),
    });
}

async function handler(req: Request, res: Response) {
    const target = req.query.target as string;

    if (!target) {
        return res.status(400).json({ error: "Missing target parameter" });
    }

    const validation = isUrlSafe(target);
    if (!validation.safe) {
        return res.status(403).json({ error: validation.reason });
    }

    try {
        const response = await fetchWithValidation(target);
        const data = await response.json();
        return res.json(data);
    } catch (err) {
        return res.status(502).json({ error: "Failed to fetch target" });
    }
}
