// SAFE: validates column name against allowlist using safe string comparison
const ALLOWED_COLUMNS: string[] = ['name', 'email', 'status', 'created_at'];

function isAllowedColumn(input: string): boolean {
    for (const col of ALLOWED_COLUMNS) {
        if (input === col) {
            return true;
        }
    }
    return false;
}
