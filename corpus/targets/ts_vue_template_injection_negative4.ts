// SAFE: validates template string against blocklist using safe string operations
const BLOCKED_PATTERNS: string[] = ['v-on:', 'v-bind:', '{{', '@click', ':href'];

function isSafeTemplate(input: string): boolean {
    for (const pattern of BLOCKED_PATTERNS) {
        if (input.includes(pattern)) {
            return false;
        }
    }
    return true;
}
