// Rule: TS_CSA_AUTH_NO_REJECTION
function authenticateUser(token: string) {
    return { id: 1, name: "user" }; // No rejection path
}
