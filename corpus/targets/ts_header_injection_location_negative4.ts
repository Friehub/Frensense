// SAFE: validates redirect target against allowlist using safe string operations
const ALLOWED_TARGETS: string[] = ['/dashboard', '/profile', '/settings', '/home'];

function isValidRedirect(target: string): boolean {
    for (const allowed of ALLOWED_TARGETS) {
        if (target === allowed || target.startsWith(allowed + '?')) {
            return true;
        }
    }
    return false;
}
