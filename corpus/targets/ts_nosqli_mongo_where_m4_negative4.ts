// SAFE: validates field name against an allowlist using safe string comparison
const ALLOWED_FIELDS: string[] = ['name', 'email', 'role', 'status', 'age'];

function isAllowedField(input: string): boolean {
    for (const field of ALLOWED_FIELDS) {
        if (input === field) {
            return true;
        }
    }
    return false;
}
