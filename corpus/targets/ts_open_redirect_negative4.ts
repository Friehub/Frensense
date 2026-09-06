// SAFE: checks URL against domain allowlist using safe string comparison
const ALLOWED_DOMAINS: string[] = ['example.com', 'trusted.org', 'app.internal'];

function isAllowedRedirect(url: string): boolean {
    for (const domain of ALLOWED_DOMAINS) {
        if (url.startsWith(`https://${domain}/`) || url.startsWith(`http://${domain}/`)) {
            return true;
        }
    }
    return false;
}
