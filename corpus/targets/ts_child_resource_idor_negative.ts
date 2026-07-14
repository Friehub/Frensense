// SAFE: child resource verified against the validated parent in the same query
async function openDispute(userId: string, orderId: string, orderLineId: string, db: DB) {
  const order = await db.prepare('SELECT id FROM orders WHERE id = ? AND user_id = ?')
    .bind(orderId, userId).first();
  if (!order) throw new Error('ORDER_NOT_FOUND');

  // SAFE: child must belong to the validated parent
  const line = await db.prepare(
    'SELECT ol.*, op.seller_id FROM order_lines ol JOIN order_packages op ON ol.package_id = op.id WHERE ol.id = ? AND op.order_id = ?'
  ).bind(orderLineId, orderId).first();
  if (!line) throw new Error('INVALID_ORDER_LINE');

  await createDispute(orderId, orderLineId, line.seller_id, db);
}

async function stopAgentRun(runId: string, session: Session, env: Env) {
  const inst = await env.AGENT_RUN_WORKFLOW.get(runId);
  const status = await inst.status();
  // SAFE: ownership verified before action
  if (status.output?._customerId !== session.customerId) {
    throw new Error('FORBIDDEN');
  }
  await inst.terminate();
}

async function deleteProjectFile(projectId: string, fileId: string, session: Session, db: DB) {
  const proj = await db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!proj || proj.owner_id !== session.customerId) throw new Error('FORBIDDEN');

  // SAFE: fileId scoped to projectId in the DELETE
  await db.prepare('DELETE FROM project_files WHERE id = ? AND project_id = ?')
    .bind(fileId, projectId).run();
}
