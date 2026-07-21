// SAFE: validates field name against allowlist using safe string comparison
const ALLOWED_FIELDS: string[] = ['name', 'email', 'age', 'status'];

function isAllowedField(input: string): boolean {
    for (const field of ALLOWED_FIELDS) {
        if (input === field) {
            return true;
        }
    }
    return false;
}
