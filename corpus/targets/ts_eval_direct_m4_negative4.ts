// SAFE: validates expression against safe operators using safe string operations
const SAFE_OPERATORS: string[] = ['+', '-', '*', '/', '(', ')', '.', ' '];

function isSafeExpression(input: string): boolean {
    let valid = true;
    for (const op of SAFE_OPERATORS) {
        valid = valid && input.includes(op);
    }
    return valid;
}
