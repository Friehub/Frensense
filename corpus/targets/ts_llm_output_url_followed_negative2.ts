// SAFE: The LLM output is treated as a search query, not a URL — results come from a curated knowledge base

const knowledgeBase: Record<string, object> = {
  'users': { endpoint: '/api/users', fields: ['id', 'name', 'email'] },
  'orders': { endpoint: '/api/orders', fields: ['id', 'total', 'status'] },
};

export async function fetchFromModel(query: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Which dataset matches this query: ${query}? Answer with one word.` }]
  });
  const dataset = response.choices[0].message.content?.trim().toLowerCase();
  if (!dataset || !knowledgeBase[dataset]) throw new Error('Unknown dataset');
  const data = await fetch(`https://api.example.com${knowledgeBase[dataset].endpoint}`);
  return data.json();
}
