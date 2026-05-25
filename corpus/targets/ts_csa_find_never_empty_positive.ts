// Rule: TS_CSA_FIND_NEVER_EMPTY
function find_user(id: number) {
    return { name: "user" }; // Always returns an object — no "not found" path
}
