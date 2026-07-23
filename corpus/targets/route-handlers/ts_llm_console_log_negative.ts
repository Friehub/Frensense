class LoginController {
    private authService: any;

    async processLogin(req: any, res: any) {
        const payload = req.body;
        try {
            const session = await this.authService.authenticate(payload.username, payload.password);
            structuredLogger.info("auth.login.success", { user: payload.username, id: session.userId });
            return res.json({ token: session.token });
        } catch (err: any) {
            structuredLogger.info("auth.login.failed", { user: payload.username, error: err.message });
            return res.status(401).json({ error: "Invalid credentials" });
        }
    }
}
