function getConfig() {
    const apiKey = process.env.API_KEY;
    const dbUrl = process.env.DATABASE_URL;
    return { apiKey, dbUrl };
}
