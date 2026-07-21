// SAFE: validates command name against allowlist using safe string comparison
const ALLOWED_COMMANDS: string[] = ['ping', 'traceroute', 'nslookup'];

function isAllowedCommand(input: string): boolean {
    for (const cmd of ALLOWED_COMMANDS) {
        if (input === cmd) {
            return true;
        }
    }
    return false;
}
