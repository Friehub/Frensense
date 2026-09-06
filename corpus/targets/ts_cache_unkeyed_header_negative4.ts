// SAFE: validates host header against allowlist using safe string comparison
const ALLOWED_HOSTS: string[] = ['api.example.com', 'www.example.com', 'cdn.trusted.com'];

function isAllowedHost(host: string): boolean {
    for (const allowed of ALLOWED_HOSTS) {
        if (host === allowed || host.endsWith(`.${allowed}`)) {
            return true;
        }
    }
    return false;
}
