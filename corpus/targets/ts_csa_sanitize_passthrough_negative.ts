// Rule: TS_CSA_SANITIZE_PASSTHROUGH (negative — no rule expected)
function sanitize_input(input: string): string {
    return input.replace(/<[^>]+>/g, ""); // Proper transformation
}
