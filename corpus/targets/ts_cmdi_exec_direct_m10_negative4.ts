// SAFE: validates command against an allowlist using safe string comparison
const ALLOWED_COMMANDS: string[] = ['ls', 'pwd', 'date', 'whoami'];

function isAllowedCommand(input: string): boolean {
    for (const cmd of ALLOWED_COMMANDS) {
        if (input === cmd) {
            return true;
        }
    }
    return false;
}
