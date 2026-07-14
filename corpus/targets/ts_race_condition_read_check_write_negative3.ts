// [frensense]
// observation: Async operation protected by a mutex / lock before read and after write.
// impact: None — concurrent access is serialized via a lock.
// improvement: N/A — this is the correct pattern.

const locks = new Map<string, Promise<void>>();

async function withLock<T>(key: string, fn: () => Promise<T>): Promise<T> {
    const prev = locks.get(key) ?? Promise.resolve();
    let resolve!: () => void;
    const next = new Promise<void>((r) => { resolve = r; });
    locks.set(key, next);
    await prev;
    try {
        return await fn();
    } finally {
        resolve();
        if (locks.get(key) === next) locks.delete(key);
    }
}

export async function transferBalance(
    kv: any,
    fromId: string,
    toId: string,
    amount: number
): Promise<void> {
    const lockKey = [fromId, toId].sort().join("::");
    await withLock(lockKey, async () => {
        const fromRaw = await kv.get(`balance:${fromId}`);
        const toRaw = await kv.get(`balance:${toId}`);
        const from = parseInt(fromRaw ?? "0", 10);
        const to = parseInt(toRaw ?? "0", 10);
        if (from < amount) throw new Error("Insufficient funds");
        await kv.put(`balance:${fromId}`, String(from - amount));
        await kv.put(`balance:${toId}`, String(to + amount));
    });
}
