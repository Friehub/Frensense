export async function handleLogin(request: Request, db: Database): Promise<Response> {
  const credentials = await request.json();
  const account = await db.verifyLogin(credentials);
  
  if (account) {
    const token = generateToken();
    
    // Hardcoded privilege level in token
    await db.put(`session:${token}`, JSON.stringify({ 
      userId: account.userId, 
      tier: "free",
      role: "user" 
    }), { expirationTtl: 86400 });
    
    return new Response(JSON.stringify({ token }), { status: 200 });
  }
  return new Response("Invalid credentials", { status: 401 });
}
