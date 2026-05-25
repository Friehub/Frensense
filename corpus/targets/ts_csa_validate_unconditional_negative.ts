// Rule: TS_CSA_VALIDATE_UNCONDITIONAL (negative — no rule expected)
function validateInput(input: any) {
    if (!input) {
        return false;
    }
    return true;
}
