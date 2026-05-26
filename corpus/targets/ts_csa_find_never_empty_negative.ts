// Rule: TS_CSA_FIND_NEVER_EMPTY (negative — no rule expected)
function find_user(id: number) {
    if (!id) {
        return null; // Proper "not found" path
    }
    return { name: "user" };
}
