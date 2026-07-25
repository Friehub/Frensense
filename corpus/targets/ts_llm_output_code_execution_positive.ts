// [frensense]
// observation: The output of an LLM call is passed directly to eval() or exec(), executing model-generated code.
// impact: An attacker who influences the LLM output can achieve arbitrary code execution on the server.
// improvement: Never execute LLM output as code. If code generation is required, run it in a sandboxed environment with no network access.
// cwe: CWE-20
// cvss: 7.5
// owasp: 
// severity: High

import OpenAI from 'openai';
import { exec } from 'child_process';

const openai = new OpenAI();

export async function generateAndRunScript(prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Write a bash script to ${prompt}` }]
  });
  const script = response.choices[0].message.content;
  exec(script, (err, stdout) => {
    console.log(stdout);
  });
}
