import { URL } from "url";

function validateImageUrl(url: string): { valid: boolean; error?: string } {
    try {
        const parsed = new URL(url);

        if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
            return { valid: false, error: "Only HTTP(S) URLs are allowed" };
        }

        const urlStr = parsed.toString();
        const decoded = decodeURIComponent(urlStr);
        if (decoded !== urlStr) {
            return { valid: false, error: "URL contains encoded recursive parameters" };
        }

        const params = Array.from(parsed.searchParams.entries());
        for (const [key, value] of params) {
            if (typeof value === "string" && value.includes(`${key}=`)) {
                return { valid: false, error: `"${key}" parameter cannot be recursive` };
            }
        }

        const hostname = parsed.hostname;
        if (!hostname || hostname === "localhost" || hostname === "127.0.0.1") {
            return { valid: false, error: "Internal URLs are not allowed" };
        }

        return { valid: true };
    } catch {
        return { valid: false, error: "Invalid URL format" };
    }
}

function handleImageRequest(req: Request, res: Response) {
    const imageUrl = req.query.url as string;

    if (!imageUrl) {
        return res.status(400).json({ error: "Missing url parameter" });
    }

    const validation = validateImageUrl(imageUrl);
    if (!validation.valid) {
        return res.status(400).json({ error: validation.error });
    }

    return fetchImage(imageUrl);
}
