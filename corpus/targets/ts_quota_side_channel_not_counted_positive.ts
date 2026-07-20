// [frensense]
// observation: Certain side-channel operations (e.g., MCP tool calls, web searches, or file reads) consume quota but the quota deduction is not implemented, making them free.
// impact: Users can exploit side-channel features (like web browsing or code execution via MCP) without any quota deduction, bypassing the billing system for those operations.
// improvement: Apply the same quota deduction to all operations that consume billable resources, not just the primary feature.

export async function handleToolCall(userId: string, toolName: string, args: any, env: Env) {
  // Deducts quota for LLM calls but NOT for MCP tool calls
  if (toolName === 'llm_chat') {
    const quota = await checkAndDeductQuota(userId, env);
    if (!quota.allowed) throw new Error('Quota exceeded');
  }

  // VULNERABLE: MCP tools like web_search, code_interpreter
  // are not deducted from quota, making them free to use
  const result = await env.MCP.invoke(toolName, args);
  return result;
}
