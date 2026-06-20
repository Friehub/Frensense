// Rule: TS_CSA_VALIDATE_UNCONDITIONAL (negative — no rule expected)
// A function that properly validates and rejects invalid input.

interface Credentials {
    username: string;
    password: string;
    mfaCode?: string;
}

interface ValidationResult {
    valid: boolean;
    errors: string[];
}

function validateCredentials(input: unknown): ValidationResult {
    const errors: string[] = [];

    if (!input || typeof input !== 'object') {
        return { valid: false, errors: ["Input must be an object"] };
    }

    const creds = input as Record<string, unknown>;

    if (!creds.username || typeof creds.username !== 'string') {
        errors.push("Username is required and must be a string");
    } else if (creds.username.length < 3) {
        errors.push("Username must be at least 3 characters");
    }

    if (!creds.password || typeof creds.password !== 'string') {
        errors.push("Password is required");
    } else if (creds.password.length < 8) {
        errors.push("Password must be at least 8 characters");
    } else if (!/[A-Z]/.test(creds.password as string)) {
        errors.push("Password must contain at least one uppercase letter");
    } else if (!/[0-9]/.test(creds.password as string)) {
        errors.push("Password must contain at least one number");
    }

    if (creds.mfaCode !== undefined) {
        if (typeof creds.mfaCode !== 'string') {
            errors.push("MFA code must be a string");
        } else if (creds.mfaCode.length !== 6) {
            errors.push("MFA code must be 6 digits");
        }
    }

    return { valid: errors.length === 0, errors };
}

function validateApiKey(key: string): boolean {
    if (!key || typeof key !== 'string') {
        return false;
    }

    if (key.length < 20) {
        return false;
    }

    if (!key.startsWith("sk_live_") && !key.startsWith("pk_live_")) {
        return false;
    }

    return true;
}
