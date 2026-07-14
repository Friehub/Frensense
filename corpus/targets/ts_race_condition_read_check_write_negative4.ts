// [frensense]
// observation: A synchronous variable check and update.
// impact: None — since Node.js execution is single-threaded for synchronous code, there is no chance of a race condition between the read and the update.
// improvement: N/A — this is the correct pattern.

export function applyDiscount(cart: any, discount: number): boolean {
    // Read
    const currentTotal = cart.total;
    
    // Check
    if (currentTotal < discount) {
        return false;
    }
    
    // Write
    cart.total = currentTotal - discount;
    return true;
}

export function consumeToken(rateLimiter: any, tokens: number): boolean {
    const available = rateLimiter.tokens;
    if (available >= tokens) {
        rateLimiter.tokens -= tokens;
        return true;
    }
    return false;
}
