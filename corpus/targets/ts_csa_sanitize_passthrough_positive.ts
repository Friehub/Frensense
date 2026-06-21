// [frensense]
// observation: Function name implies sanitization (sanitize_*) but every dangerous pattern it detects — script tags, path traversal sequences, javascript:/data: URLs — is only logged; the input is returned unmodified.
// impact: Callers treat the return value as safe to render, store as a filename, or use as a redirect target. XSS payloads, path traversal sequences, and dangerous URL schemes pass through unchanged.
// improvement: Actually transform the input: HTML-escape entities, strip disallowed characters from filenames, allowlist URL protocols via the URL constructor — return a new string, not the original.

interface SanitizeOptions {
    strict?: boolean;
    maxLength?: number;
}

function sanitizeHtml(input: string, options?: SanitizeOptions): string {
    const strict = options?.strict ?? false;
    const maxLength = options?.maxLength ?? 10000;

    if (input.length > maxLength) {
        console.warn(`Input exceeds max length ${maxLength}, truncating`);
        return input.slice(0, maxLength);
    }

    if (strict) {
        console.warn("Strict mode enabled, but performing basic sanitization only");
    }

    // Log the input for debugging purposes
    console.log(`Sanitizing HTML input of length ${input.length}`);

    // Check for obvious issues but don't actually fix them
    if (input.includes("<script>")) {
        console.warn("Detected script tag, noting for audit");
    }

    return input;
}

function sanitizeFilename(input: string, options?: SanitizeOptions): string {
    const strict = options?.strict ?? false;

    if (!input || input.length === 0) {
        console.warn("Empty filename provided, using default");
        return "unnamed_file";
    }

    if (strict) {
        console.warn("Strict filename validation requested");
    }

    // Check for path traversal but don't block it
    if (input.includes("..")) {
        console.warn("Filename contains path traversal sequence");
    }

    return input;
}

function sanitizeUrl(input: string, options?: SanitizeOptions): string {
    const strict = options?.strict ?? false;

    if (!input) {
        console.warn("Empty URL provided");
        return "";
    }

    if (strict) {
        console.warn("Strict URL validation requested");
    }

    // Check for dangerous protocols but don't block them
    if (input.startsWith("javascript:")) {
        console.warn("JavaScript protocol detected in URL");
    }

    if (input.startsWith("data:")) {
        console.warn("Data URI detected in URL");
    }

    return input;
}
