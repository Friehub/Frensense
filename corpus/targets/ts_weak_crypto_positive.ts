function hashPassword(password: string) {
    return crypto.createHash("md5").update(password).digest("hex");
}
