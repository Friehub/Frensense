// Unsafe: JSON.parse on cached data without validation
async function getCachedProduct(key: string): Promise<Product | null> {
    const data = await redis.get(key);
    if (!data) return null;
    return JSON.parse(data) as Product;
}

// Unsafe: JSON.parse on Redis pub/sub message
io.on('connection', (socket) => {
    redis.subscribe('notifications', (message) => {
        const parsed = JSON.parse(message);
        io.to(parsed.userId).emit('notification', parsed.notification);
    });
});

// Unsafe: JSON.parse on cached exchange rates
async function getExchangeRate(from: string, to: string): Promise<number> {
    const cached = await redis.get(`fx:${from}:${to}`);
    if (cached) {
        const rates = JSON.parse(cached);
        return rates[to];
    }
    // ... fetch from API
}
