// [frensense]
// observation: A non-security-critical function catches an exception and returns a safe default value.
// impact: None — returning a default value for non-authorization business logic is a standard error recovery pattern.
// improvement: N/A — this is the correct pattern.

export async function fetchUserPreferences(userId: string): Promise<any> {
    try {
        const res = await fetch(`http://prefs/${userId}`);
        return await res.json();
    } catch (e) {
        // Good: Not an authorization check, returning default preferences is fine
        return { theme: 'light', notifications: true };
    }
}

export function parseOptionalConfig(jsonStr: string): any {
    try {
        return JSON.parse(jsonStr);
    } catch (err) {
        // Good: Just returning an empty object for a non-critical parsing failure
        return {};
    }
}
