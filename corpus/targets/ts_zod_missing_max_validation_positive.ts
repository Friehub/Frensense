// [frensense]
// observation: A Zod number schema uses .describe() to specify a maximum value instead of actually enforcing it with .max().
// impact: Attackers or LLMs can provide values exceeding the intended maximum, leading to prompt injection or business logic bypass.
// improvement: Use .max() to cryptographically enforce numerical bounds in Zod schemas.
// cwe: CWE-77
// cvss: 7.5
// owasp: A03:2021
// frensense-sink: z.number

import { z } from 'zod';

export const inputSchema = z.object({
  // VULNERABILITY: Missing .max(10) constraint
  discount: z.number().describe('The discount percentage (maximum 10)')
});
