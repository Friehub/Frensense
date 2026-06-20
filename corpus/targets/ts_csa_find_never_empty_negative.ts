// Rule: TS_CSA_FIND_NEVER_EMPTY (negative — no rule expected)
// A function that properly returns null when user is not found.

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
        return null;
    }

    const includeDeleted = options?.includeDeleted ?? false;
    const maxResults = options?.maxResults ?? 10;

    const cached = userCache.get(id);
    if (cached) {
        if (cached.status === "deleted" && !includeDeleted) {
            return null;
        }
        return cached;
    }

    const queryResult = database.query("SELECT * FROM users WHERE id = ?", [id]);

    if (queryResult.length === 0) {
        return null;
    }

    const user = queryResult[0];
    if (user.status === "deleted" && !includeDeleted) {
        return null;
    }

    return user;
}
