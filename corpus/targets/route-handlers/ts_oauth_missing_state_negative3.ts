// SAFE: OAuth state parameter is generated, stored in session, and validated on callback
import crypto from 'crypto';

export function initiateLogin(req: Request, res: Response): void {
    const state = crypto.randomUUID();
    req.session.oauthState = state;
    const params = new URLSearchParams({
        client_id: 'CLIENT_ID',
        redirect_uri: 'https://app.example.com/callback',
        response_type: 'code',
        scope: 'openid email',
        state: state
    });
    res.redirect('https://provider.com/oauth/authorize?' + params.toString());
}

export async function handleCallback(req: Request, res: Response): Promise<void> {
    const { code, state } = req.query;
    if (!state || state !== req.session.oauthState) {
        res.status(401).json({ error: 'State mismatch, possible CSRF' });
        return;
    }
    delete req.session.oauthState;
    const tokens = await exchangeCode(code as string);
    req.session.userId = tokens.sub;
    res.redirect('/dashboard');
}
