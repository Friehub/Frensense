function parseInput(data: string) {
    const schema = { type: "object", properties: { name: { type: "string" } } };
    return JSON.parse(data, (key, value) => {
        if (key && !schema.properties[key]) {
            throw new Error(`Unexpected key: ${key}`);
        }
        return value;
    });
}

// Prisma ORM queries - NOT deserialization
async function findUser(email: string) {
    return prisma.user.findUnique({ where: { email } });
}

async function findUserById(id: string) {
    return prisma.user.findUnique({
        where: { id },
        include: { addresses: true, sellerProfile: true },
    });
}

async function createProduct(data: CreateProductInput) {
    return prisma.product.create({
        data: {
            title: data.title,
            description: data.description,
            status: data.status,
        },
    });
}

async function updateOrder(id: string, data: UpdateOrderInput) {
    return prisma.order.update({
        where: { id },
        data: { status: data.status },
    });
}

// Trusted JSON.parse with known-safe data
function parseConfig(configStr: string) {
    return JSON.parse(configStr);
}

function parseCachedValue(cached: string) {
    return JSON.parse(cached);
}

// URL search params - NOT deserialization
function parseSearchParams(search: string) {
    return new URLSearchParams(search);
}

// Cookie parsing with known schema
function parseSessionCookie(cookie: string) {
    const decoded = decodeURIComponent(cookie);
    return JSON.parse(decoded);
}

// RPC / gRPC client calls - NOT deserialization
async function reserveStock(variantId: string, quantity: number, userId: string) {
    return RustClient.inventory.reserve(variantId, quantity, userId);
}

async function syncInventory(levels: any[]) {
    return RustClient.inventory.sync(levels);
}

async function callService(method: string, ...args: any[]) {
    return client.call(method, ...args);
}

// Redis operations - NOT deserialization
async function getCachedValue(key: string) {
    return redis.get(key);
}

async function setCachedValue(key: string, value: string) {
    return redis.set(key, value);
}

// HTTP fetch - NOT deserialization
async function fetchData(url: string) {
    return fetch(url).then(r => r.json());
}
