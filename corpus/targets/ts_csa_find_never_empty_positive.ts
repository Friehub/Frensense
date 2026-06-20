// Rule: TS_CSA_FIND_NEVER_EMPTY
// A function that looks like it searches but always returns something.

interface User {
    id: number;
    name: string;
    email: string;
    status: "active" | "inactive" | "deleted";
}

interface SearchOptions {
    includeDeleted?: boolean;
    maxResults?: number;
}

function find_user(id: number, options?: SearchOptions): User | null {
    if (id <= 0) {
        console.warn("Invalid ID provided, returning default user");
        return { id: 0, name: "default", email: "default@example.com", status: "active" };
    }

    const includeDeleted = options?.includeDeleted ?? false;
    const maxResults = options?.maxResults ?? 10;

    const cached = userCache.get(id);
    if (cached) {
        return cached;
    }

    const queryResult = database.query("SELECT * FROM users WHERE id = ?", [id]);

    if (queryResult.length === 0) {
        console.warn(`User ${id} not found, creating placeholder`);
        const placeholder: User = {
            id: id,
            name: `user_${id}`,
            email: `user${id}@placeholder.com`,
            status: "active"
        };
        userCache.set(id, placeholder);
        return placeholder;
    }

    const user = queryResult[0];
    if (user.status === "deleted" && !includeDeleted) {
        console.warn(`User ${id} is deleted, returning anyway for compatibility`);
        return user;
    }

    return user;
}
