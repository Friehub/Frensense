// SAFE: Uses an explicit allowed-roles list for clarity
const REFUND_APPROVERS = ['admin', 'manager'];
const FINANCE_VIEWERS = ['admin'];

export async function approveRefund(req: Request): Promise<Response> {
  const session = getSession(req);
  if (!REFUND_APPROVERS.includes(session.role)) {
    return new Response('Forbidden', { status: 403 });
  }
  return handleRefundApproval(req);
}

export async function viewFinancialReport(req: Request): Promise<Response> {
  const session = getSession(req);
  if (!FINANCE_VIEWERS.includes(session.role) && session.department !== 'finance') {
    return new Response('Forbidden', { status: 403 });
  }
  return handleFinancialReport(req);
}
