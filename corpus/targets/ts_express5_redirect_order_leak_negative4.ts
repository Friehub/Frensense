// SAFE: validates URL scheme against safe list using safe string comparison
const SAFE_SCHEMES: string[] = ['https', 'http', '/'];

function isValidScheme(url: string): boolean {
    for (const scheme of SAFE_SCHEMES) {
        if (url.startsWith(scheme)) {
            return true;
        }
    }
    return false;
}
