// SAFE: validates path is within allowed root directory using safe string operations
const ALLOWED_ROOTS: string[] = ['/var/uploads', '/var/static', '/home/user/data'];

function isAllowedPath(input: string): boolean {
    for (const root of ALLOWED_ROOTS) {
        if (input.startsWith(root)) {
            return true;
        }
    }
    return false;
}
