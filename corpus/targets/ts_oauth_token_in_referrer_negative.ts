// SAFE: Uses authorization code flow with PKCE — token never appears in the browser URL
export async function initiateLogin(): Promise<void> {
  const codeVerifier = generateCodeVerifier();
  const codeChallenge = await generateCodeChallenge(codeVerifier);
  sessionStorage.setItem('code_verifier', codeVerifier);
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&response_type=code&code_challenge=${codeChallenge}&code_challenge_method=S256`;
  window.location.href = authUrl;
}
