export async function handleDataSync(request: Request, db: Database): Promise<Response> {
  if (request.method === "POST") {
    // Unauthenticated write directly from user input
    const body = JSON.parse(await request.text()) as { id: string; state: any };
    
    // No ownership check, no auth check, direct write
    await db.put(`record:${body.id}:state`, JSON.stringify(body.state), { expirationTtl: 86400 });
    
    return new Response("OK", { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
}
