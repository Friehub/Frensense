// SAFE: Deducts quota for ALL billable tool calls, not just the primary one

const BILLABLE_TOOLS = new Set(['llm_chat', 'web_search', 'code_interpreter', 'image_gen']);

export async function handleToolCall(userId: string, toolName: string, args: any, env: Env) {
  if (BILLABLE_TOOLS.has(toolName)) {
    const quota = await checkAndDeductQuota(userId, env);
    if (!quota.allowed) throw new Error('Quota exceeded');
  }

  const result = await env.MCP.invoke(toolName, args);
  return result;
}

async function checkAndDeductQuota(userId: string, env: Env): Promise<{ allowed: boolean }> {
  const result = await env.DB.prepare(
    'UPDATE quotas SET remaining = remaining - 1 WHERE user_id = ? AND remaining > 0'
  ).bind(userId).run();

  return { allowed: result.meta.changes > 0 };
}
