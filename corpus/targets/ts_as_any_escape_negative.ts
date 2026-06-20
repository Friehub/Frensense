interface UserPayload {
    name: string;
    email: string;
    age: number;
}

interface ValidatedUser {
    name: string;
    email: string;
    age: number;
    isValid: boolean;
}

function validateAndTransform(raw: unknown): ValidatedUser {
    if (typeof raw !== "object" || raw === null) {
        throw new Error("Input must be an object");
    }

    const input = raw as Record<string, unknown>;

    if (typeof input.name !== "string" || input.name.trim().length === 0) {
        throw new Error("name must be a non-empty string");
    }

    if (typeof input.email !== "string" || !input.email.includes("@")) {
        throw new Error("email must be a valid email address");
    }

    if (typeof input.age !== "number" || input.age < 0 || input.age > 150) {
        throw new Error("age must be a number between 0 and 150");
    }

    return {
        name: input.name.trim(),
        email: input.email.toLowerCase(),
        age: Math.floor(input.age),
        isValid: true,
    };
}

function processData(items: unknown[]): ValidatedUser[] {
    const results: ValidatedUser[] = [];
    for (const item of items) {
        try {
            const validated = validateAndTransform(item);
            results.push(validated);
        } catch {
            // Skip invalid items
        }
    }
    return results;
}

function formatOutput(user: ValidatedUser): string {
    return JSON.stringify({
        name: user.name,
        email: user.email,
        age: user.age,
    });
}
