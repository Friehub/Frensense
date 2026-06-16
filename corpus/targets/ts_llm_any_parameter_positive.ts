interface UserPayload {
    id: number;
    name: string;
    email: string;
    role: 'admin' | 'user';
}

function processUser(raw: any): UserPayload {
    const id = raw.id;
    const name = raw.name;
    const email = raw.email;
    const role = raw.role;

    if (typeof id !== 'number' || id <= 0) {
        throw new Error('Invalid user ID');
    }
    if (typeof name !== 'string' || name.length === 0) {
        throw new Error('Name is required');
    }
    if (typeof email !== 'string' || !email.includes('@')) {
        throw new Error('Invalid email');
    }
    if (role !== 'admin' && role !== 'user') {
        throw new Error('Invalid role');
    }

    return { id, name, email, role };
}
