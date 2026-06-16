function sanitizeHtml(input: string): string {
    return input
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#x27;');
}

function sanitizeFilename(input: string): string {
    return input.replace(/[^a-zA-Z0-9._-]/g, '_').slice(0, 255);
}

function sanitizeUrl(input: string): string {
    try {
        const url = new URL(input);
        if (!['http:', 'https:'].includes(url.protocol)) throw new Error('bad protocol');
        return url.toString();
    } catch {
        return '';
    }
}

function sanitizeQuery(input: string): string {
    return encodeURIComponent(input);
}
