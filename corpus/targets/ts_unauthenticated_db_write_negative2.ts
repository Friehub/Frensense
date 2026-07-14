// [frensense]
// observation: Database write operation protected by an authentication/authorization check.
// impact: None — write is safe because the user is authenticated and authorized.
// improvement: N/A — this is the correct pattern.

export async function handleAdminUpdate(request: Request, db: Database): Promise<Response> {
  if (request.method === "POST") {
    // 1. Authenticate user
    const token = request.headers.get("Authorization")?.split(" ")[1];
    if (!token) return new Response("Unauthorized", { status: 401 });
    
    const user = await verifyToken(token);
    if (!user) return new Response("Unauthorized", { status: 401 });

    // 2. Authorize user (Admin only)
    if (!user.roles.includes("admin")) {
        return new Response("Forbidden", { status: 403 });
    }

    // 3. Safe write
    const body = JSON.parse(await request.text()) as { id: string; state: any };
    await db.put(`record:${body.id}:state`, JSON.stringify(body.state));
    
    return new Response("OK", { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
}

async function verifyToken(token: string): Promise<any> {
    // Stub implementation
    return { id: "123", roles: ["admin"] };
}
