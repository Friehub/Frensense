// SAFE: Every tool execution is gated by an authorization check against the caller's role

import OpenAI from 'openai';

const openai = new OpenAI();

async function authorize(userId: string, action: string): Promise<boolean> {
  const user = await getUserRole(userId);
  return user.role === 'admin';
}

export async function agentHandler(userId: string, prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: prompt }],
    tools: [
      {
        type: 'function',
        function: {
          name: 'deleteUser',
          description: 'Delete a user account',
          parameters: { type: 'object', properties: { targetUserId: { type: 'string' } } }
        }
      }
    ]
  });
  const toolCall = response.choices[0].message.tool_calls?.[0];
  if (toolCall?.function.name === 'deleteUser') {
    if (!await authorize(userId, 'deleteUser')) throw new Error('Unauthorized');
    const args = JSON.parse(toolCall.function.arguments);
    await deleteUser(args.targetUserId);
  }
}
