function sanitizeHtml(input: string): string {
    return input
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#x27;");
}

function sanitizeFilename(input: string): string {
    const normalized = input.normalize("NFC");
    return normalized
        .replace(/[^a-zA-Z0-9._-]/g, "_")
        .replace(/^\.+/, "")
        .slice(0, 255);
}

function sanitizeUrl(input: string): string {
    try {
        const url = new URL(input);
        const allowedProtocols = ["http:", "https:"];
        if (!allowedProtocols.includes(url.protocol)) {
            throw new Error(`Protocol ${url.protocol} not allowed`);
        }
        url.hostname.split(".").forEach((part) => {
            if (part.length === 0) throw new Error("Invalid hostname");
        });
        return url.toString();
    } catch {
        return "";
    }
}

function sanitizeQuery(input: string): string {
    const decoded = decodeURIComponent(input);
    const encoded = encodeURIComponent(decoded);
    return encoded
        .replace(/%20/g, "+")
        .replace(/%27/g, "'")
        .replace(/%22/g, '"');
}

function sanitizeCss(input: string): string {
    return input
        .replace(/expression\(/gi, "")
        .replace(/javascript:/gi, "")
        .replace(/url\(/gi, "url(")
        .replace(/behavior:/gi, "");
}
