// SAFE: PKCE implemented with S256 code challenge and verifier stored in session
import crypto from 'crypto';

export function initiateLogin(req: Request, res: Response): void {
    const codeVerifier = crypto.randomBytes(32).toString('base64url');
    const codeChallenge = crypto.createHash('sha256').update(codeVerifier).digest('base64url');
    req.session.codeVerifier = codeVerifier;
    const params = new URLSearchParams({
        client_id: 'ID',
        response_type: 'code',
        redirect_uri: 'https://app.example.com/callback',
        code_challenge: codeChallenge,
        code_challenge_method: 'S256'
    });
    res.redirect('https://provider.com/oauth/authorize?' + params.toString());
}

export async function handleCallback(req: Request, res: Response): Promise<void> {
    const { code } = req.query;
    const codeVerifier = req.session.codeVerifier;
    if (!codeVerifier) {
        res.status(401).json({ error: 'Missing code verifier' });
        return;
    }
    delete req.session.codeVerifier;
    const tokens = await exchangeCodeWithPkce(code as string, codeVerifier);
    req.session.userId = tokens.sub;
    res.redirect('/dashboard');
}
