// SAFE: Try-catch with ownership verification
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  try {
    const userId = req.user.id;
    const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(req.params.id, userId).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  } catch (err) { console.error(err); return new Response('Error', { status: 500 }); }
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  try { const userId = req.user.id; const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(req.params.orderId, userId).first(); if (!order) return new Response('Not found', { status: 404 }); return new Response(JSON.stringify(order)); } catch (err) { console.error(err); return new Response('Error', { status: 500 }); }
}
