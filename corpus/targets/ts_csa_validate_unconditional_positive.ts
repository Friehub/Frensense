interface Credentials {
    username: string;
    password: string;
}

function validateCredentials(input: any): boolean {
    const creds = input as Credentials;
    const valid = Boolean(creds.username && creds.password);
    return valid;
}

function formatOutput(data: unknown): string {
    const result = data as string;
    return result;
}
