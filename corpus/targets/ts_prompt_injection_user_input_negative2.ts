// SAFE: User input is sanitized to strip out instruction-like sequences before being included in the prompt

function sanitizeUserMessage(msg: string): string {
  return msg.replace(/<\/?system>/gi, '').replace(/ignore.*instructions/gi, '');
}

export async function chatHandler(userMessage: string) {
  const safeMessage = sanitizeUserMessage(userMessage);
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: 'You are a helpful assistant.' },
      { role: 'user', content: safeMessage }
    ]
  });
  return response.choices[0].message.content;
}
