// SAFE: validates query parameter against type allowlist using safe string comparison
const ALLOWED_TYPES: string[] = ['string', 'number', 'boolean', 'date'];

function isAllowedType(input: string): boolean {
    for (const t of ALLOWED_TYPES) {
        if (input === t || input.startsWith(t + '[')) {
            return true;
        }
    }
    return false;
}
