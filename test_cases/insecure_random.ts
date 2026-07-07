function createSessionId() {
    const charset = 'abcdef0123456789';
    let sessionId = '';
    for (let i = 0; i < 16; i++) {
        const randomIndex = Math.floor(Math.random() * charset.length);
        sessionId += charset[randomIndex];
    }
    return sessionId;
}
