// SAFE: Uses correct OR logic for role checking (admin OR manager can approve)
export async function approveRefund(req: Request): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin' && session.role !== 'manager') {
    return new Response('Forbidden', { status: 403 });
  }
  return handleRefundApproval(req);
}

export async function viewFinancialReport(req: Request): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin' || session.department !== 'finance') {
    return new Response('Forbidden', { status: 403 });
  }
  return handleFinancialReport(req);
}
