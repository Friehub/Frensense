// SAFE: Privilege fetched from license service on every request, validated with JWT
export async function handleLogin(request: Request, db: Database): Promise<Response> {
  const credentials = await request.json();
  const account = await db.verifyLogin(credentials);
  if (account) {
    const license = await fetchLicense(account.userId);
    const token = jwt.sign({ sub: account.userId, tier: license.tier, role: license.role }, process.env.JWT_SECRET!, { expiresIn: '15m' });
    return new Response(JSON.stringify({ token }), { status: 200 });
  }
  return new Response("Invalid credentials", { status: 401 });
}

async function fetchLicense(userId: string): Promise<{ tier: string; role: string }> {
  const res = await fetch(`http://license-service/users/${userId}/license`);
  return res.json();
}
