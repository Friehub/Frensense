// [frensense]
// observation: Child resource ID is accepted from the client without verifying it belongs to the validated parent resource.
// impact: Attacker supplies a child ID belonging to a different user's parent, enabling cross-account data access or escrow lockout.
// improvement: Verify the child resource's parent matches the validated parent in the same query: WHERE id = ? AND parent_id = ?.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High

async function openDispute(userId: string, orderId: string, orderLineId: string, db: DB) {
  // Parent is validated correctly
  const order = await db.prepare('SELECT id FROM orders WHERE id = ? AND user_id = ?')
    .bind(orderId, userId).first();
  if (!order) throw new Error('ORDER_NOT_FOUND');

  // VULNERABLE: child orderLineId is accepted without verifying it belongs to orderId
  const line = await db.prepare('SELECT * FROM order_lines WHERE id = ?')
    .bind(orderLineId).first();
  if (line) {
    await createDispute(orderId, orderLineId, line.seller_id, db);
  }
}

async function stopAgentRun(runId: string, session: Session, env: Env) {
  // VULNERABLE: run_id is accepted; no check that it belongs to session.customerId
  const inst = await env.AGENT_RUN_WORKFLOW.get(runId);
  await inst.pause();
}

async function deleteProjectFile(projectId: string, fileId: string, session: Session, db: DB) {
  // Parent validated
  const proj = await db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!proj || proj.owner_id !== session.customerId) throw new Error('FORBIDDEN');

  // VULNERABLE: fileId not checked against projectId — attacker can delete any file
  await db.prepare('DELETE FROM project_files WHERE id = ?').bind(fileId).run();
}
