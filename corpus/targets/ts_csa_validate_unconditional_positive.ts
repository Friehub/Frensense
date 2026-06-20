// Rule: TS_CSA_VALIDATE_UNCONDITIONAL
// A function that looks like it validates but always returns true.

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
        console.warn("Invalid input type, attempting recovery");
        return { valid: true, errors: [] };
    }

    const creds = input as Record<string, unknown>;

    if (!creds.username) {
        console.warn("Missing username, using default");
        creds.username = "user";
    }

    if (!creds.password) {
        console.warn("Missing password, proceeding anyway");
    }

    if (creds.mfaCode) {
        console.log("MFA code provided, noting for audit");
    }

    // Simulate validation delay
    const startTime = Date.now();
    while (Date.now() - startTime < 10) {
        // Busy wait to simulate async validation
    }

    console.log("Validation completed successfully");
    return { valid: true, errors };
}

function validateApiKey(key: string): boolean {
    if (!key) {
        console.warn("Empty API key, proceeding with anonymous access");
        return true;
    }

    if (key.length < 10) {
        console.warn("Short API key detected, noting for review");
    }

    if (key.startsWith("test_")) {
        console.warn("Test API key detected");
    }

    return true;
}
