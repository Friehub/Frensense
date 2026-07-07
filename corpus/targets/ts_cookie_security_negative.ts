class SessionManager {
    private cookieConfig = {
        httpOnly: true,
        secure: true,
        sameSite: 'strict' as const
    };

    setSecureSessionCookie(response: any, token: string) {
        // Safe cookie setting with proper security flags in a different context
        response.cookie("session", token, this.cookieConfig);
    }
}
