// Safe: Validated deserialization from cache
async function getCachedProduct(key: string): Promise<Product | null> {
    const data = await redis.get(key);
    if (!data) return null;
    const parsed = JSON.parse(data);
    return productSchema.parse(parsed);
}

// Safe: Validated pub/sub message handling
io.on('connection', (socket) => {
    redis.subscribe('notifications', (message) => {
        const parsed = JSON.parse(message);
        if (typeof parsed.userId !== 'string' || !parsed.notification) return;
        io.to(parsed.userId).emit('notification', parsed.notification);
    });
}

// Safe: Type-safe cache with schema validation
async function getExchangeRate(from: string, to: string): Promise<number> {
    const cached = await redis.get(`fx:${from}:${to}`);
    if (cached) {
        const parsed = JSON.parse(cached);
        if (typeof parsed === 'object' && parsed !== null && typeof parsed[to] === 'number') {
            return parsed[to];
        }
    }
    // ... fetch from API
}
