// [frensense]
// observation: The LLM system prompt is embedded as a constant in client-bundled code or returned from a server endpoint without authentication, making it readable in the browser
// impact: Attackers can extract proprietary system prompts, discover guardrail instructions, or craft targeted prompt injection bypasses
// improvement: Keep system prompts exclusively server-side, never expose them in API responses or client bundles
// cwe: CWE-20
// cvss: 7.5
// owasp: 
// severity: High

function getSystemPrompt(): string {
  return 'You are a financial advisor. Never recommend stocks under $5. Always disclose risks.';
}
