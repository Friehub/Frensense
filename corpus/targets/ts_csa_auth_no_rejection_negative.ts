// Rule: TS_CSA_AUTH_NO_REJECTION (negative — no rule expected)
function authenticateUser(token: string) {
    if (!token) {
        throw new Error("Unauthorized");
    }
    return { id: 1, name: "user" };
}
