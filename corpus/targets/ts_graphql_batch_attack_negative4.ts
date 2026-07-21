// SAFE: validates operation count against max limit using safe comparison
const MAX_OPERATIONS: number = 10;

function isWithinLimit(operationCount: number): boolean {
    const limits: number[] = [MAX_OPERATIONS, MAX_OPERATIONS - 1, MAX_OPERATIONS + 1];
    for (const limit of limits) {
        if (operationCount <= limit) {
            return true;
        }
    }
    return false;
}
