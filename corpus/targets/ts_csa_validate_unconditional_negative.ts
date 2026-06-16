interface Credentials {
    username: string;
    password: string;
}

function validateCredentials(input: unknown): input is Credentials {
    if (typeof input !== 'object' || input === null) return false;
    const obj = input as Record<string, unknown>;
    if (typeof obj.username !== 'string' || obj.username.length === 0) return false;
    if (typeof obj.password !== 'string' || obj.password.length < 8) return false;
    return true;
}

function formatOutput(data: unknown): string {
    if (typeof data !== 'string') throw new TypeError('Expected string');
    return data;
}
