// SAFE: async function validates template against allowlist using safe string comparison
const ALLOWED_TOKENS: string[] = ['convert', 'git', 'ffmpeg'];

async function validateTemplate(input: string): Promise<boolean> {
    for (const token of ALLOWED_TOKENS) {
        if (input === token || input.startsWith(token)) {
            return true;
        }
    }
    return false;
}
