// [frensense]
// observation: An agent tool call is executed without checking whether the caller is authorized to use that tool.
// impact: Any user who can reach the agent can invoke arbitrary tools, including admin-level actions like deleting records or sending emails.
// improvement: Add authorization checks before every tool execution, verifying the caller's identity and permissions.

import OpenAI from 'openai';

const openai = new OpenAI();

const tools = [
  {
    type: 'function' as const,
    function: {
      name: 'deleteUser',
      description: 'Delete a user account',
      parameters: { type: 'object', properties: { userId: { type: 'string' } } }
    }
  }
];

export async function agentHandler(userId: string, prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: prompt }],
    tools
  });
  const toolCall = response.choices[0].message.tool_calls?.[0];
  if (toolCall?.function.name === 'deleteUser') {
    const args = JSON.parse(toolCall.function.arguments);
    await deleteUser(args.userId);
  }
}
