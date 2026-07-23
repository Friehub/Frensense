// [frensense]
// observation: Role-based access control uses incorrect logical operators (&& instead of || or vice versa), allowing users to bypass intended restrictions.
// impact: Users may gain access to resources they should not have, or be denied access they should have, due to incorrect boolean logic in role checking.
// improvement: Carefully review the intended access policy and use the correct logical operators. Write explicit tests for role combinations.

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
