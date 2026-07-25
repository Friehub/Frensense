// [frensense]
// observation: The application requests all available OAuth scopes instead of the minimum required for its functionality.
// impact: The application gains unnecessary permissions to user data. If the application is compromised, the attacker inherits broad access to users' resources.
// improvement: Follow the principle of least privilege — request only the scopes actually needed for the application's features.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

export function initiateLogin(req: Request, res: Response): void {
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&scope=openid%20email%20profile%20photos%20files%20calendar%20contacts%20drive%20admin&response_type=code`;
  res.redirect(authUrl);
}
