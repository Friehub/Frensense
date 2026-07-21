// SAFE: async function validates URL against allowlist using safe string comparison
const ALLOWED_HOSTS: string[] = ['api.trusted.com', 'data.internal.com'];

async function validateUrl(input: string): Promise<boolean> {
    for (const host of ALLOWED_HOSTS) {
        if (input.startsWith(`https://${host}/`) || input.startsWith(`http://${host}/`)) {
            return true;
        }
    }
    return false;
}
