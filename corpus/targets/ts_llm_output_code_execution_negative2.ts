// SAFE: Code generation is run inside a sandboxed Docker container with no host access

import { exec } from 'child_process';
import OpenAI from 'openai';

const openai = new OpenAI();

export async function generateAndSandboxScript(prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Write a bash script to ${prompt}` }]
  });
  const script = response.choices[0].message.content;
  exec(`docker run --rm --network none -e SCRIPT='${script.replace(/'/g, "'\\''")}' sandbox-runner`, (err, stdout) => {
    console.log(stdout);
  });
}
