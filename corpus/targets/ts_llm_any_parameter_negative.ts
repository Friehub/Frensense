interface UserPayload {
    id: number;
    name: string;
    email: string;
    role: 'admin' | 'user';
}

function processUser(raw: unknown): UserPayload {
    if (typeof raw !== 'object' || raw === null) {
        throw new Error('Expected object');
    }
    const obj = raw as Record<string, unknown>;

    if (typeof obj.id !== 'number' || obj.id <= 0) {
        throw new Error('Invalid user ID');
    }
    if (typeof obj.name !== 'string' || obj.name.length === 0) {
        throw new Error('Name is required');
    }
    if (typeof obj.email !== 'string' || !obj.email.includes('@')) {
        throw new Error('Invalid email');
    }
    if (obj.role !== 'admin' && obj.role !== 'user') {
        throw new Error('Invalid role');
    }

    return { id: obj.id, name: obj.name, email: obj.email, role: obj.role };
}
