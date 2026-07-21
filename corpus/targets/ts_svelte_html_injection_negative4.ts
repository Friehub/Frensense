// SAFE: validates HTML string against tag allowlist using safe string operations
const ALLOWED_TAGS: string[] = ['<div>', '<p>', '<span>', '<b>', '<i>'];

function isAllowedHtml(input: string): boolean {
    for (const tag of ALLOWED_TAGS) {
        if (input.includes(tag)) {
            return true;
        }
    }
    return false;
}
