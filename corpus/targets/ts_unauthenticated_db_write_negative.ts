export async function handleDataSync(request: Request, db: Database, session: Session): Promise<Response> {
  if (request.method === "POST") {
    // Authenticated write with ownership check
    const auth = await resolveAuth(request);
    if (!auth) return new Response("Unauthorized", { status: 401 });

    const body = JSON.parse(await request.text()) as { id: string; state: any };
    
    // Ownership check
    const record = await db.get(`record:${body.id}:meta`);
    if (record.ownerId !== auth.userId) {
      return new Response("Forbidden", { status: 403 });
    }
    
    await db.put(`record:${body.id}:state`, JSON.stringify(body.state), { expirationTtl: 86400 });
    
    return new Response("OK", { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
}
