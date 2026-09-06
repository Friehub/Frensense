// SAFE: validates table name against allowlist using safe string comparison
const ALLOWED_TABLES: string[] = ['users', 'posts', 'comments'];

function isAllowedTable(input: string): boolean {
    for (const table of ALLOWED_TABLES) {
        if (input === table) {
            return true;
        }
    }
    return false;
}
