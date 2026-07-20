// SAFE: Tools are registered with required roles, and the agent only exposes tools the caller is permitted to use

import OpenAI from 'openai';

const openai = new OpenAI();

const toolRegistry = {
  admin: [
    { type: 'function' as const, function: { name: 'deleteUser', description: 'Delete a user account', parameters: { type: 'object', properties: { userId: { type: 'string' } } } } }
  ],
  user: [
    { type: 'function' as const, function: { name: 'getProfile', description: 'Get user profile', parameters: { type: 'object', properties: {} } } }
  ]
};

export async function agentHandler(userId: string, prompt: string) {
  const user = await getUserRole(userId);
  const tools = toolRegistry[user.role] || toolRegistry.user;
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
  if (toolCall?.function.name === 'getProfile') {
    return getProfile(userId);
  }
}
