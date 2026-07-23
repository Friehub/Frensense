// SAFE: Try-catch with ownership check via JOIN
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  try {
    const invoice = await db.prepare('SELECT i.* FROM invoices i JOIN users u ON i.user_id = u.id WHERE i.id = ? AND u.id = ?').bind(req.params.id, req.user.id).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  } catch (err) { return new Response('Error', { status: 500 }); }
}
