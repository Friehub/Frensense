// [frensense]
// observation: A multi-step workflow (REVIEW → APPROVED → SHIPPED) allows transitioning directly from REVIEW to SHIPPED, bypassing the required approval step.
// impact: An attacker can ship content or orders without managerial approval, bypassing the business process control.
// improvement: Enforce approval state as a prerequisite for shipping in the state machine logic.

export async function shipContent(contentId: string, env: Env) {
  // VULNERABLE: ships directly from REVIEW, skipping the APPROVED step
  const content = await env.DB.prepare(
    'SELECT status FROM content WHERE id = ?'
  ).bind(contentId).first();

  if (!content) throw new Error('Not found');

  if (content.status === 'REVIEW') {
    await env.DB.prepare(
      'UPDATE content SET status = ? WHERE id = ?'
    ).bind('SHIPPED', contentId).run();

    await notifySubscribers(contentId, env);
  }
}
