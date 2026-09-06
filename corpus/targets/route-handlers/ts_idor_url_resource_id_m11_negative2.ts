// SAFE: Conditional branch with ownership check via JOIN
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  if (req.params.id) {
    const invoice = await db.prepare('SELECT i.* FROM invoices i JOIN users u ON i.user_id = u.id WHERE i.id = ? AND u.id = ?').bind(req.params.id, req.user.id).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  }
  return new Response('Missing id', { status: 400 });
}
