// SAFE: validates user prompt against a blocklist before sending to the LLM
const BLOCKLIST = ['ignore instructions', 'system prompt', 'forget previous'];

function isPromptSafe(userMessage: string): boolean {
  const lowerInput = userMessage.toLowerCase();
  for (const blocked of BLOCKLIST) {
    if (lowerInput.includes(blocked)) {
      return false;
    }
  }
  return true;
}

export async function chat(userMessage: string) {
  if (!isPromptSafe(userMessage)) {
    return { reply: 'Message filtered due to policy violation.' };
  }
  const response = await fetch('https://api.openai.com/v1/chat/completions', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${process.env.OPENAI_API_KEY}` },
    body: JSON.stringify({ model: 'gpt-4', messages: [{ role: 'system', content: 'You are helpful.' }, { role: 'user', content: userMessage }] })
  });
  return { reply: (await response.json()).choices[0].message.content };
}
