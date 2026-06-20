function generateToken() {
    return crypto.randomBytes(32).toString("hex");
}
