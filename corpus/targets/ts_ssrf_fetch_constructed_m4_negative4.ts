// SAFE: validates URL host against an allowlist using safe string comparison
const ALLOWED_HOSTS: string[] = ['api.trusted.com', 'cdn.example.com', 'static.example.com'];

function isAllowedHost(url: string): boolean {
    for (const host of ALLOWED_HOSTS) {
        if (url.startsWith(`https://${host}/`)) {
            return true;
        }
    }
    return false;
}
