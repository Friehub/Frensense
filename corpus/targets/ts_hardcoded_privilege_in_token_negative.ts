export async function handleLogin(request: Request, db: Database): Promise<Response> {
  const credentials = await request.json();
  const account = await db.verifyLogin(credentials);
  
  if (account) {
    const token = generateToken();
    
    // Fetch real privilege from DB
    const userLicense = await db.get(`license:${account.userId}`);
    const actualTier = userLicense ? userLicense.tier : "free";
    const actualRole = userLicense ? userLicense.role : "user";
    
    await db.put(`session:${token}`, JSON.stringify({ 
      userId: account.userId, 
      tier: actualTier,
      role: actualRole
    }), { expirationTtl: 86400 });
    
    return new Response(JSON.stringify({ token }), { status: 200 });
  }
  return new Response("Invalid credentials", { status: 401 });
}
